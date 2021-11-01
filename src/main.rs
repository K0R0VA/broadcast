mod broadcaster;
mod event;
mod messages;
mod room;
mod session;
mod state;

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{post, web::Data, App, HttpResponse, HttpServer, Result};
use messages::CreateRoom;
use serde::Serialize;
use state::State;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .app_data(Data::new(State::default().start()))
            .wrap(cors)
            .service(create_room)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

#[derive(Serialize)]
pub struct RoomId(Uuid);

#[post("/create-room")]
async fn create_room(room_name: String, state: Data<Addr<State>>) -> Result<HttpResponse> {
    if let Ok(Some(id)) = state.send(CreateRoom { room_name }).await {
        return Ok(HttpResponse::Created().json(RoomId(id)));
    };
    HttpResponse::Created().await
}
