// src/p2p/network.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// サーバーとして起動し、他のノードからの接続を待ち受ける関数
pub fn start_server(port: &str) {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to bind to port");
    println!("P2P Server started on {}", address);

    // 新しい接続が来るたびに、スレッド（別行動の部下）を作って処理する
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New peer connected!");
                
                // 別スレッドでデータの読み込みを開始（メインの処理を止めないため）
                thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    // 相手からのデータを読み込む
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        if bytes_read > 0 {
                            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                            println!("Received message from peer: {}", message);
                        }
                    }
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

/// クライアントとして、指定したポートの他のノードにメッセージを送る関数
pub fn connect_to_peer(port: &str, message: &str) {
    let address = format!("127.0.0.1:{}", port);
    
    // 他のノードに接続を試みる
    if let Ok(mut stream) = TcpStream::connect(&address) {
        println!("Successfully connected to peer at {}", address);
        
        // メッセージを送信
        stream.write_all(message.as_bytes()).expect("Failed to write to stream");
        println!("Sent message: {}", message);
    } else {
        println!("Could not connect to peer at {}", address);
    }
}