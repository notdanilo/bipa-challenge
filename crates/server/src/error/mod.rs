use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Self::Io(io) => actix_web::HttpResponse::InternalServerError().body(io.to_string()),
            Self::Reqwest(reqwest) => actix_web::HttpResponse::InternalServerError().body(reqwest.to_string()),
            Self::Deserialization(serde_json) => actix_web::HttpResponse::InternalServerError().body(serde_json.to_string()),
            Self::InvalidTimestamp => actix_web::HttpResponse::InternalServerError().body("Invalid timestamp".to_string()),
            Self::Sql(sql) => actix_web::HttpResponse::InternalServerError().body(sql.to_string()),
            Self::Migration(migration) => actix_web::HttpResponse::InternalServerError().body(migration.to_string()),
            Self::Dotenv(dotenv) => actix_web::HttpResponse::InternalServerError().body(dotenv.to_string()),
            Self::ParseInt(parse_int) => actix_web::HttpResponse::InternalServerError().body(parse_int.to_string()),
        }
    }
}
