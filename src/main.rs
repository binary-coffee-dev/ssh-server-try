use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;

use russh::keys::ssh_key::rand_core::OsRng;
use russh::keys::ssh_key::LineEnding::LF;
use russh::keys::{Certificate, PublicKey};
use russh::server::{Auth, Handler, Msg, Server as _, Session};
use russh::*;
use tokio::sync::Mutex;

mod view;
use crate::view::actions::{map_key, Action};
use crate::view::view_trait::EventResult;
use view::view_root::ViewRoot;
use view::view_trait::ViewTrait;
use view::*;

#[tokio::main]
async fn main() {
    // Generate or load the private key BEGIN
    let private_key_file = env::var("BC_PRIVATE_KEY")
        .map_or("./cert/private_key.pem".to_string(), |v| {
            String::from(v.as_str())
        });
    let private_key;
    if std::path::Path::new(private_key_file.as_str()).exists() {
        private_key =
            keys::PrivateKey::from_openssh(std::fs::read(private_key_file).unwrap()).unwrap();
    } else {
        private_key = keys::PrivateKey::random(&mut OsRng, keys::Algorithm::Ed25519).unwrap();
        let mut file = std::fs::File::create(private_key_file).unwrap();
        file.write_all(private_key.to_openssh(LF).unwrap().as_str().as_bytes())
            .unwrap();
    }
    // Generate or load the private key END

    let config = server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![private_key],
        preferred: Preferred {
            ..Preferred::default()
        },
        ..Default::default()
    };

    let config = Arc::new(config);
    let mut sh = Server {
        clients: Arc::new(Mutex::new(HashMap::new())),
        view_root: ViewRoot::new(),
        id: 0,
    };
    println!("Starting server...");
    sh.run_on_address(config, ("0.0.0.0", 2222)).await.unwrap();
}

#[derive(Clone)]
struct Server {
    clients: Arc<Mutex<HashMap<usize, (ChannelId, server::Handle)>>>,
    view_root: ViewRoot,
    id: usize,
}

impl Server {
    async fn _post(&mut self, data: CryptoVec) {
        // not needed if we don't want to propagate the message to other clients
        let mut clients = self.clients.lock().await;
        for (id, (channel, ref mut s)) in clients.iter_mut() {
            if *id != self.id {
                let _ = s.data(*channel, data.clone()).await;
            }
        }
    }

    async fn close_action(&mut self) -> Result<(), Error> {
        // not needed if we don't want to propagate the message to other clients
        let mut clients = self.clients.lock().await;
        for (id, (channel, ref mut s)) in clients.iter_mut() {
            if *id == self.id {
                let mut screen = clear_screen!().as_str().to_string();
                screen.push_str(exit_alt_screen!().as_str());
                let _ = s.data(*channel, screen.into()).await;
                let _ = s.close(*channel).await;
                break;
            }
        }
        Ok(())
    }

    async fn draw(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
        action: Option<Action>,
    ) -> Result<(), Error> {
        // clean the screen and move the cursor to the top left
        let mut screen = clear_screen!().to_string();
        screen.push_str(move_cursor!().as_str());

        match action {
            Some(act) => match self.view_root.event(&act) {
                Some(EventResult::Quite) => {
                    return self.close_action().await;
                }
                _ => {}
            },
            None => {}
        }

        // paint the screen
        let mut screen_drawed = vec![
            " ".repeat(self.view_root.details.width as usize);
            self.view_root.details.height as usize
        ];
        self.view_root.draw(&mut screen_drawed, None);

        // post operations
        let operations = self.view_root.post_operations(None);

        // apply the operations and draw the screen
        screen.push_str(to_screen_text(&screen_drawed, operations).as_str());

        // set the cursor position
        let cursor_pos = self.view_root.cursor_position(None).unwrap_or((1, 1));
        screen.push_str(move_cursor!(cursor_pos.0, cursor_pos.1).as_str());

        session.data(channel, screen.as_bytes().into())?;
        Ok(())
    }
}

impl server::Server for Server {
    type Handler = Self;

    fn new_client(&mut self, _: Option<SocketAddr>) -> Self::Handler {
        let s = self.clone();
        self.id += 1;
        s
    }

    fn handle_session_error(&mut self, _error: <Self::Handler as Handler>::Error) {
        eprintln!("Session error: {:?}", _error);
        _ = self.close_action();
    }
}

impl server::Handler for Server {
    type Error = russh::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        {
            let mut clients = self.clients.lock().await;
            clients.insert(self.id, (channel.id(), session.handle()));
        }
        Ok(true)
    }

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _public_key: &PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_openssh_certificate(
        &mut self,
        _user: &str,
        _certificate: &Certificate,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.data(channel, enter_alt_screen!().into())?;
        self.view_root.redimension(col_width, row_height);
        self.draw(channel, session, None).await
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.view_root.redimension(col_width, row_height);
        self.draw(channel, session, None).await
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        // println!("Data received: {:?}", data);

        let action = map_key(data);
        match action {
            Some(Action::Eof) => {
                return self.close_action().await;
            }
            _ => {}
        }

        self.draw(channel, session, action).await
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        _ = self.close_action();
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}
