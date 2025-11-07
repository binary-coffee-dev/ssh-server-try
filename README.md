# SSH server try

This project aims to provide a fully functional version of BinaryCoffee accessible through the terminal via SSH.

## Start project

```bash
cargo run
```

## Setup project in VPS

1. Change the default SSH port in `/etc/ssh/sshd_config`:

   ```bash
   Port 20
   ```

2. Check the firewall settings to allow the new SSH port:

   ```bash
   sudo ufw allow 20/tcp
   sudo ufw reload
   ```

   > **Note:** Check first if the firewall is enabled with the command `sudo ufw status`.

3. Restart the SSH service:

   ```bash
   sudo systemctl restart sshd && sudo systemctl restart ssh
   ```
   This steps should allow you to connect to your VPS using the new SSH port 20 and free the port 22 to be used by the
   current project.


4. Redirect the SSH port 22 to the project port (e.g., 2222):

   Modify the file `/etc/nginx/nginx.conf` and add the following code block:

   ```nginx
   stream {
      server {
         listen 22;
         proxy_connect_timeout 5s;
         proxy_timeout 600s;
         proxy_pass 127.0.0.1:2222;
      }
   }
   ```

5. Check the NGINX configuration and restart the service:

   ```bash
   # validate nginx configuration
   sudo nginx -t
   # restart nginx service
   sudo systemctl restart nginx
   ```

## ToDo

- [ ] Render markdown into the terminal

## License

- [LGPL v2.1](./LICENSE.md)

## Utils

- [Bash prompts](https://tldp.org/HOWTO/Bash-Prompt-HOWTO/index.html)
