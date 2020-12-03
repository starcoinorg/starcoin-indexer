use clap::Clap;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::Url;
use elasticsearch::{BulkParts, CreateParts, Elasticsearch, IndexParts};

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.1.0", author = "Starcoin Core Dev <dev@starcoin.org>")]
pub struct Options {
    #[clap(long, about = "es url")]
    es_url: Url,
    #[clap(long, about = "starcoin node rpc url")]
    node_url: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Options = Options::parse();
    let transport = elasticsearch::http::transport::TransportBuilder::new(
        SingleNodeConnectionPool::new(opts.es_url.clone()),
    )
    .build()?;

    let es = Elasticsearch::new(transport);
    es.index(IndexParts::Index("asd"));
    // es.indices().put_settings();
    // es.bulk(BulkParts::Index("abf")).body();
    println!("opts: {:?}", &opts);
    Ok(())
}
