use crate::{database::Database, prelude::*};
use crate::web::Node;
use actix_web::web;
use actix_web::{App, HttpServer, get, web::Json};

pub struct WebServer {
    address: String,
    port: u16,
}

impl WebServer {
    pub fn from_env() -> Result<Self> {
        let address = dotenv::var("SERVER_ADDRESS")?;
        let port = dotenv::var("SERVER_PORT")?.parse::<u16>()?;
        Ok(Self { address, port })
    }

    pub fn new(address: impl ToString, port: u16) -> Self {
        let address = address.to_string();
        Self { address, port }
    }

    pub async fn run(self, database: &Database) -> Result<()> {
        let database = database.clone();
        tracing::info!("Starting web server on {}:{}", self.address, self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(database.clone()))
                .service(nodes)
        })
        .bind((self.address, self.port))?
        .run()
        .await?;

        Ok(())
    }
}

#[get("/nodes")]
#[tracing::instrument(skip(database))]
async fn nodes(database: web::Data<Database>) -> actix_web::Result<Json<Vec<Node>>> {
    tracing::debug!("Handling /nodes request");
    let nodes = database.read_nodes().await?;
    let nodes = nodes.into_iter().map(Node::from).collect::<Vec<_>>(); // TODO: Avoid conversion
    Ok(Json(nodes))
}
