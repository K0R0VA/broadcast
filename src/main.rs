mod broadcaster_context;
mod recipient;
mod broadcaster;
mod event;
mod messages;
mod room;
mod session;
mod state;

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{Data, Path, Payload},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use messages::CreateRoom;
use serde::Serialize;
use session::Session;
use state::State;
use uuid::Uuid;

use crate::messages::GetRoomName;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Data::new(State::default().start());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .app_data(Data::clone(&state))
            .wrap(cors)
            .service(create_room)
            .service(get_room_name)
            .service(start)
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
    HttpResponse::BadRequest().await
}

#[get("/room/{room_id}/name")]
async fn get_room_name(room_id: Path<Uuid>, state: Data<Addr<State>>) -> Result<HttpResponse> {
    if let Ok(Some(name)) = state
        .send(GetRoomName {
            room_id: room_id.into_inner(),
        })
        .await
    {
        return Ok(HttpResponse::Ok().json(name));
    }
    HttpResponse::NotFound().await
}

#[get("/start")]
async fn start(
    state: Data<Addr<State>>,
    req: HttpRequest,
    payload: Payload,
) -> Result<HttpResponse> {
    let id = Uuid::new_v4();
    let session = Session::new(state.get_ref().clone(), id);
    if let Ok(response) = actix_web_actors::ws::start(session, &req, payload) {
        return Ok(response);
    }
    HttpResponse::BadGateway().await
}
