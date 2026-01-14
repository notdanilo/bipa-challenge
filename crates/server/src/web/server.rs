use crate::web::Node;
use actix_web::{App, HttpServer, Result, get, web::Json};

pub struct WebServer {
    server: String,
    port: u16,
}

impl WebServer {
    pub fn new(server: impl ToString, port: u16) -> Self {
        let server = server.to_string();
        Self { server, port }
    }

    pub async fn run(self) -> crate::error::Result<()> {
        HttpServer::new(|| {
            App::new().service(nodes)
        })
        .bind((self.server, self.port))?
        .run()
        .await?;

        Ok(())
    }
}

#[get("/nodes")]
async fn nodes() -> Result<Json<Vec<Node>>> {
    let nodes = crate::data::mempool::Node::get().await?;
    let nodes = nodes.into_iter().map(Node::from).collect::<Vec<_>>();
    Ok(Json(nodes))
}
