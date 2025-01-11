use ssh2::Session;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};

pub struct SSHConnection {
    sessions: HashMap<u16, Session>,
}

impl SSHConnection {
    pub fn new() -> Self {
        SSHConnection {
            sessions: HashMap::new(),
        }
    }

    pub fn connect(&mut self, addr: &str, user: &str, password: &str) {
        let tcp = TcpStream::connect(addr).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_password(user, password).unwrap();
        assert!(sess.authenticated());

        self.sessions.insert(1, sess);
    }

    pub fn shell(&mut self, id: u16) -> io::Result<()> {
        let mut channel = self.sessions.get(&id).unwrap().channel_session().unwrap();

        let channel_arc = Arc::new(Mutex::new(channel));
        let (tx, mut rx) = mpsc::channel();

        tokio::spawn(async move {
            while let Ok(Some(msg)) = rx.recv() {
                let mut channel = channel_arc.lock().unwrap();
                channel.write_all(msg.as_bytes()).unwrap();
                channel.flush().unwrap();
            }
        });

        // 从服务器接收数据并发送到 WebSocket
        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                let mut channel = channel_arc.lock().unwrap();
                let size = channel.read(&mut buffer).unwrap();
                if size == 0 {
                    break;
                }
                tx.send(buffer[..size].to_vec()).await.unwrap();
            }
        });

        Ok(())
    }
}

#[test]
fn test_ssh_connection() {
    let mut conn = SSHConnection::new();
    conn.connect("47.109.56.194:22", "root", "9fjSiG9@jF77x&ze9f158e4");

    conn.shell(1).unwrap()


    // let mut channel = sess.channel_session().unwrap();
    // channel.shell().unwrap();
    //
    // channel.write_all(b"ls\n").unwrap();
    // channel.send_eof().unwrap();
    //
    // let mut s = String::new();
    // channel.read_to_string(&mut s).unwrap();
    // println!("{}", s);
    //
    // channel.close().expect("cannot close channel");
    // channel.wait_close().unwrap();
}
