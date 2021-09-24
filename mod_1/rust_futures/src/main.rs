extern crate rust_futures;

use failure;

use tokio::net::TcpListener;
use tokio::prelude::*;

use futures::compat::{AsyncRead01CompatExt, AsyncWrite01CompatExt};
use futures::stream::StreamExt;
use futures::future::{FutureExt, TryFutureExt};
use futures::io::AsyncWriteExt;

fn main() -> Result<(), failure::Error> {
    let listener = TcpListener::bind(&"127.0.0.1:8092".parse()?)?;
    let s_future = listener.incoming()
    .map_err(|e| eprintln!("Accept failure {:?}", e))
    .for_each(|sock| {
        let f03 = async move {
            let (s_in, s_out) = sock.split();
            let mut read_stream = rust_futures::ReadStream::new(s_in.compat());

            let mut s_out = s_out.compat();
            while let Some(s) = read_stream.next().await {
                if s.len() == 0 {
                    return;
                }
                println!("s == {}", s);
                let s = format!("You said \"{}\"\n", s.trim_end());
                if let Err(_e) = s_out.write_all(s.as_bytes()).await {
                    return;
                }
            }
        };
        tokio::spawn(f03.unit_error().boxed().compat());
        Ok(())
    });

    tokio::run(s_future);

    Ok(())
}