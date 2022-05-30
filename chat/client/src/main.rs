use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::fs;
use std::process::Command;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 128;


fn read_file() -> String{
    let filename = "/home/lorenzo/fizzbuzz";
    println!("In file {}", filename);
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    return contents;
}

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    let result = Command::new("ls")
                    .args(&["~/"])
                    .status()
                    .unwrap();

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let mut msg;
        if buff.trim().to_string() == "read" {
            msg = read_file();
        } else {
            msg = buff.trim().to_string();

        }
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("bye bye!");

}
