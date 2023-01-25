use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use azure_core::auth::TokenCredential;
use azure_identity::{DefaultAzureCredential};
use url::Url;
use std::env;
use std::error::Error;


// #[tokio::main2]
async fn authenticate() -> Result<(), Box<dyn Error>> {
    let credential = DefaultAzureCredential::default();
    let response = credential
        .get_token("https://management.azure.com")
        .await?;

    let subscription_id = env::var("AZURE_SUBSCRIPTION_ID")?;
    let url = Url::parse(&format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01",
        subscription_id))?;
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", response.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", response);

    Ok(())
}

#[get("/")]
async fn oidc() -> impl Responder {
    let auth_result = authenticate().await;
    if auth_result.is_ok() {
        HttpResponse::Ok().body(format!("oidc ok"))
    }
    else {
        HttpResponse::Forbidden().body(format!("oidc failed"))
    }
}

#[get("/unsecure")]
async fn healcheck() -> impl Responder {
    HttpResponse::Ok().body("{\"status\": \"UP\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(oidc)
            .service(healcheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
