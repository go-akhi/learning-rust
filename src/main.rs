use learning_rust::deploy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    deploy()?.await
}