use web_server::server::http_server;
use web_server::utils::print_info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_info();
    http_server().unwrap().await
}
