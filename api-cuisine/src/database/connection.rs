use dotenv::dotenv;
use std::env;
use elasticsearch::{
    Elasticsearch,
    Error,
    http::transport::Transport,
    auth::Credentials,
};
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;

pub fn client() -> Result<Elasticsearch, Error>{
    dotenv().ok();
    let connect = env::var("URL_DATABASE").unwrap();
    let url = Url::parse(&connect).unwrap();
    let pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(pool).disable_proxy().build()?;
    let client_elastic = Elasticsearch::new(transport);
    Ok(client_elastic)
}