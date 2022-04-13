use dotenv::dotenv;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;
use std::env;

pub fn client() -> anyhow::Result<Elasticsearch> {
    dotenv().ok();
    let connect = env::var("URL_DATABASE")?;
    let url = Url::parse(&connect)?;
    let pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(pool).disable_proxy().build()?;
    let client_elastic = Elasticsearch::new(transport);
    Ok(client_elastic)
}
