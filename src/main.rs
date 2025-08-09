use std::net::SocketAddr;
use warp::Filter;
mod blockchain;
use blockchain::BlockchainManager;
mod gotas;
use gotas::{GotasClient, NFTMetadata};

#[tokio::main]
async fn main() {
    // Inicializar o gerenciador de blockchains
    let blockchain_manager = BlockchainManager::new();
    
    // Inicializar o cliente do gotas.social
    let gotas_client = GotasClient::new(
        "https://gotas.social".to_string(),
        None, // Adicione uma API key se necessário
    );
    
    // Configuração do servidor
    let addr: SocketAddr = ([0, 0, 0, 0], 3030).into();
    
    // Rota básica para servir arquivos estáticos
    let static_files = warp::path("static")
        .and(warp::fs::dir("static/"));
    
    // Rota básica para a página inicial
    let index = warp::get()
        .and(warp::path::end())
        .map(|| warp::reply::html("<h1>NFT PassKey DApp</h1><p>Servidor em execução</p>"));
    
    // Rota para obter a lista de blockchains suportadas
    let blockchain_list = warp::get()
        .and(warp::path("blockchains"))
        .map(move || {
            let manager = blockchain_manager.clone();
            let blockchains = manager.list_blockchains();
            warp::reply::json(&blockchains)
        });
    
    // Rota para criar um NFT
    let create_nft = warp::post()
        .and(warp::path("nfts"))
        .and(warp::body::json())
        .and_then(move |metadata: NFTMetadata| {
            let client = gotas_client.clone();
            async move {
                match client.create_nft(metadata, "ethereum").await {
                    Ok(nft) => Ok(warp::reply::json(&nft)),
                    Err(e) => Err(warp::reject::custom(NFTError(e.to_string()))),
                }
            }
        })
        .recover(|err: warp::Rejection| async move {
            if let Some(nft_error) = err.find::<NFTError>() {
                Ok(warp::reply::with_status(
                    nft_error.0.clone(),
                    warp::http::StatusCode::BAD_REQUEST,
                ))
            } else {
                Err(err)
            }
        });
    
    // Combinar rotas
    let routes = static_files.or(index).or(blockchain_list).or(create_nft);
    
    println!("Servidor iniciado em http://{}", addr);
    warp::serve(routes)
        .run(addr)
        .await;
}

#[derive(Debug)]
struct NFTError(String);

impl warp::reject::Reject for NFTError {}