use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use tokio::net::UdpSocket;
use turn::{
    auth::{generate_auth_key, AuthHandler},
    relay::relay_static::RelayAddressGeneratorStatic,
    server::{
        config::{ConnConfig, ServerConfig},
        Server,
    },
};
use webrtc_util::Error;

pub async fn create_turn_server() -> anyhow::Result<Server> {
    let username = "portal";
    let realm = "private";
    let password = "password";
    let mut credentials_map = HashMap::new();
    let key = generate_auth_key(username, realm, password);
    credentials_map.insert(username.to_owned(), key);
    let conn = Arc::new(UdpSocket::bind(format!("0.0.0.0:443")).await?);
    let server = Server::new(ServerConfig {
        conn_configs: vec![ConnConfig {
            conn,
            relay_addr_generator: Box::new(RelayAddressGeneratorStatic {
                relay_address: IpAddr::from_str("192.168.0.7")?,
                address: "0.0.0.0".to_owned(),
                net: Arc::new(webrtc_util::vnet::net::Net::new(None)),
            }),
        }],
        realm: realm.to_owned(),
        auth_handler: Arc::new(MyAuthHandler::new(credentials_map)),
        channel_bind_timeout: Duration::from_secs(0),
    }).await?;
    Ok(server)
}

struct MyAuthHandler {
    cred_map: HashMap<String, Vec<u8>>,
}

impl MyAuthHandler {
    fn new(cred_map: HashMap<String, Vec<u8>>) -> Self {
        MyAuthHandler { cred_map }
    }
}
impl AuthHandler for MyAuthHandler {
    fn auth_handle(
        &self,
        username: &str,
        _realm: &str,
        _src_addr: SocketAddr,
    ) -> Result<Vec<u8>, turn::Error> {
        if let Some(pw) = self.cred_map.get(username) {
            Ok(pw.to_vec())
        } else {
            Err(turn::Error::Util(Error::ErrBindFailed))
        }
    }
}
