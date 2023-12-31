use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use qrcode_generator::QrCodeEcc;
use std::io::Read;

#[derive(serde::Deserialize)]
struct QrRequest {
    text: String,
}

#[post("/qr")]
async fn generate_qr(info: web::Json<QrRequest>) -> impl Responder {
    let file_path = "tmp/file_output.png";
    qrcode_generator::to_png_to_file(&info.text, QrCodeEcc::Low, 1024, file_path).unwrap();

    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    HttpResponse::Ok().content_type("image/png").body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(generate_qr))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
