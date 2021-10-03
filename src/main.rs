mod broadcaster;

use actix_web::{
    post,
    web::{self, Path},
    App, HttpResponse, HttpServer, Result,
};
use broadcast_context::BroadcastContext;

use crate::broadcaster::Broadcaster;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(start_broadcast))
        .bind("192.168.0.7:8081")?
        .run()
        .await
}

#[post("/room/{id}/start")]
async fn start_broadcast(description: web::Bytes, id: Path<i32>) -> Result<HttpResponse> {
    let line = std::str::from_utf8(&description)?;
    let (_, response) = BroadcastContext::<Broadcaster>::create_with_addr(line, Broadcaster)
        .await.unwrap();
    let response = HttpResponse::Ok().body(response).await?;
    Ok(response.into())
}