use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;

/*
https://unix.stackexchange.com/questions/246436/how-to-set-a-dynamic-message-of-the-day-motd-in-debian-jessie-8-2-for-ssh
 */
fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "password").unwrap();
    assert!(sess.authenticated());

    if !sess.authenticated() {
        println!("Authentication failed");
        return;
    }

    let mut channel = sess.channel_session().unwrap();
    channel.shell().unwrap();

    channel.write_all(b"shell").unwrap();
    channel.send_eof().unwrap();

    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel.close().expect("cannot close channel");
    channel.wait_close().unwrap();
}
