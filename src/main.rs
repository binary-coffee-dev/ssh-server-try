use std::collections::HashMap;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;

use russh::keys::ssh_key::rand_core::OsRng;
use russh::keys::{Certificate, PublicKey};
use russh::server::{Auth, Handler, Msg, Server as _, Session};
use russh::*;
use tokio::sync::Mutex;

mod view;
use view::*;

#[tokio::main]
async fn main() {
    // Generate or load the private key BEGIN
    let private_key_file = "./private_key.txt";
    let private_key;
    if std::path::Path::new(private_key_file).exists() {
        private_key =
            keys::PrivateKey::from_bytes(std::fs::read(private_key_file).unwrap().as_ref())
                .unwrap();
    } else {
        private_key = keys::PrivateKey::random(&mut OsRng, keys::Algorithm::Ed25519).unwrap();
        let mut file = std::fs::File::create(private_key_file).unwrap();
        file.write_all(private_key.to_bytes().unwrap().to_vec().as_ref())
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
    async fn post(&mut self, data: CryptoVec) {
        // not needed if we don't want to propagate the message to other clients
        let mut clients = self.clients.lock().await;
        for (id, (channel, ref mut s)) in clients.iter_mut() {
            if *id != self.id {
                let _ = s.data(*channel, data.clone()).await;
            }
        }
    }

    fn exit_alt_screen(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Error> {
        let mut screen = clear_screen!().as_bytes().to_vec();
        screen.extend_from_slice(exit_alt_screen!().as_bytes());
        screen.extend_from_slice(move_cursor!().as_bytes());
        session.data(
            channel,
            CryptoVec::from(screen.to_vec()),
        )?;
        Ok(())
    }

    fn draw(&mut self, channel: ChannelId, session: &mut Session, data: Option<&[u8]>) -> Result<(), Error> {
        // clean the screen and move the cursor to the top left
        let mut screen = clear_screen!().as_bytes().to_vec();
        screen.extend_from_slice(move_cursor!().as_bytes());
        // if let Some(data) = data {
        //     screen.extend_from_slice(String::from_utf8_lossy(data).as_bytes());
        // }

        // paint the screen
        let mut screen_drawed = vec![
            "@".repeat(self.view_root.details.width as usize);
            self.view_root.details.height as usize
        ];
        self.view_root.draw(&mut screen_drawed, None);
        // println!("{}", to_screen_text(&screen_drawed));
        screen.extend_from_slice(to_screen_text(&screen_drawed).as_bytes());
        screen.extend_from_slice(move_cursor!().as_bytes());

        // self.post(data.clone()).await;
        session.data(channel, screen.into())?;
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
        println!("PTY");
        self.view_root.redimension(col_width, row_height);
        session.data(
            channel,
            CryptoVec::from(enter_alt_screen!().as_bytes().to_vec()),
        )?;
        self.draw(channel, session, None);
        Ok(())
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
        // println!("Window change {} {}", col_width, row_height);
        self.view_root.redimension(col_width, row_height);
        self.draw(channel, session, None);
        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if data == [3] {
            self.exit_alt_screen(channel, session)?;
            return Err(Error::Disconnect);
        }

        println!("Data received: {:?}", data);

        self.draw(channel, session, Some(data));
        // self.draw(channel, session, None);

        Ok(())
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.exit_alt_screen(channel, session)?;
        Ok(())
    }

    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.exit_alt_screen(channel, session)?;
        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}
