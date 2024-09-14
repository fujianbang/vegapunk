use std::io::Read;
use std::net::TcpStream;
use ssh2::Session;

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "passwd").unwrap();
    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ps -ef").unwrap();

    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}