use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

pub async fn run_server() {
    let listener = match TcpListener::bind("0.0.0.0:6142").await {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };


    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(v) => v,
            Err(_) => {
                return;
            }   
        };
        let (mut reader, _) = socket.split();
        let mut buf = String::new();

        match reader.read_to_string(&mut buf).await {
            Ok(_) => {},
            Err(_) => {
                return;
            }
        };

        println!("{:?}", buf);
    }

}