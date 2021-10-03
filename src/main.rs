mod broadcaster;
mod messages;
mod room;
mod session;
mod state;
mod event;

use actix::{Actor, Addr};
use actix_web::{post, web::Data, App, HttpResponse, HttpServer, Result};
use messages::CreateRoom;
use state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(State::default().start()))
            .service(create_room)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

#[post("/create-room")]
async fn create_room(room_name: String, state: Data<Addr<State>>) -> Result<HttpResponse> {
    let _ = state.send(CreateRoom { room_name }).await;
    HttpResponse::Created().await
}
