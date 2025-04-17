mod assets_data;
mod handle_request;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};

use handle_request::handle_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Error>(service_fn(|req| {
            println!("📥 {} {}", req.method(), req.uri().path());
            handle_request(req)
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Sunucu {} adresinde başlatıldı.", addr);

    server.await?;

    Ok(())
}
