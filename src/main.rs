use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use rust_add_demo::order_taking::{
    CHECK_ADDRESS_EXISTS, CHECK_PRODUCT_CODE_EXISTS, CREATE_ORDER_ACKNOWLEDGEMENT_LETTER,
    CheckAddressExists, CreateOrderAcknowledgementLetter, GET_PRODUCT_PRICE,
    SEND_ORDER_ACKNOWLEDGEMENT, unvalidated_order,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
