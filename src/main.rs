use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Configuração do servidor
    let addr: SocketAddr = ([0, 0, 0, 0], 3030).into();
    
    // Rota básica para servir arquivos estáticos
    let static_files = warp::path("static")
        .and(warp::fs::dir("static/"));
    
    // Rota básica para a página inicial
    let index = warp::get()
        .and(warp::path::end())
        .map(|| warp::reply::html("<h1>NFT PassKey DApp</h1><p>Servidor em execução</p>"));
    
    // Combinar rotas
    let routes = static_files.or(index);
    
    println!("Servidor iniciado em http://{}", addr);
    warp::serve(routes)
        .run(addr)
        .await;
}