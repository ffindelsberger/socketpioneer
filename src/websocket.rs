use tungstenite::{
    client::IntoClientRequest,
    connect,
    handshake::client::{generate_key, Request},
    http::{header::IntoHeaderName, HeaderValue},
    Message,
};

// In  generell we want to make as little assumptions as possible about the type we expect so for
// public apis we take something that can be turned into and Iterator here.
// But as this is not really a public api we could make it simpler and just take a slice
fn run_ws<H, N>(url: String, custome_headers: H) -> crate::Result<()>
where
    H: IntoIterator<Item = (N, String)>,
    N: IntoHeaderName,
{
    println!("Hello, world!");

    let mut req_builder = Request::builder().uri(url.clone());
    {
        let headers = req_builder
            .headers_mut()
            .expect("Bug: Failed to retreive headers from Request Builder");

        for header in custome_headers {
            headers.insert(header.0, HeaderValue::from_str(&header.1)?);
        }
    }

    let request = req_builder
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .header("Host", "localhost")
        .body(())
        .unwrap()
        .into_client_request()
        .unwrap();

    match connect(request) {
        Ok((mut wsocket, _)) => {
            // let connected_headers = HashMap::new();
            // // connected_headers.insert(k, v)
            // //stomp_rs::client::Client
            //
            // let connect = Connect::new("1.2".to_string(), "localhost");
            //
            // let frame = stomp_rs::protocol::Frame {
            //     command: protocol::ClientCommand::Connect,
            //     headers: connected_headers,
            //     body: "".to_string(),
            // };
            //
            // let mut connect_frame: stomp_rs::protocol::Frame<ClientCommand> = connect.into();
            //
            // wsocket
            //     .send(Message::Text(String::from_utf8(connect_frame.to_bytes())?))
            //     .unwrap();
            //
            //

            let _ = wsocket.send(Message::Text("Hello Echo".to_string()));
            loop {
                let msg = wsocket.read().expect("Error reading message");
                println!("Received: {}", msg);
            }

            //wsocket.send(tungstenite::Message::Frame).unwrap();
        }
        Err(error) => {
            match &error {
                tungstenite::Error::ConnectionClosed => todo!(),
                tungstenite::Error::AlreadyClosed => todo!(),
                tungstenite::Error::Io(_) => todo!(),
                tungstenite::Error::Tls(_) => todo!(),
                tungstenite::Error::Capacity(_) => todo!(),
                tungstenite::Error::Protocol(_) => todo!(),
                tungstenite::Error::WriteBufferFull(_) => todo!(),
                tungstenite::Error::Utf8 => todo!(),
                tungstenite::Error::AttackAttempt => todo!(),
                tungstenite::Error::Url(_) => todo!(),
                tungstenite::Error::Http(e) => {
                    println!("{}", String::from_utf8(e.body().clone().unwrap())?)
                }
                tungstenite::Error::HttpFormat(_) => todo!(),
            }
            return Err(Box::new(error));
        }
    }

    // if let Err(tungstenite::Error::Http(response)) = connect(request) {
    //     println!(
    //         "{}",
    //         String::from_utf8(response.body().clone().unwrap()).unwrap()
    //     );
    // }

    //let (mut wsocket, response) = connect(request);

    //    Ok(())
}

#[cfg(test)]
mod test {

    use super::run_ws;
    use crate::Result;

    #[test]
    fn test_ws() -> Result<()> {
        // let url = "ws://localhost:8080/ws/websocket".to_string();
        let headers = vec![
            (
                "Authorization",
                "Basic bHVmZnlAaW50ZXJuZXR4LmNvbTpCdWRpbW9uMSE ".to_string(),
            ),
            ("X-Domainrobot-Context", "4".to_string()),
        ];
        let url = "wss://ws.postman-echo.com/raw/".to_string();

        if let Err(error) = run_ws(url, headers) {
            return Err(error);
        }

        Ok(())
    }
}
