use actix_web::{Scope, web};

pub mod messages;
pub mod api_doc;


pub fn routes() -> Scope {
    web::scope("/api").service(messages::routes())
}
