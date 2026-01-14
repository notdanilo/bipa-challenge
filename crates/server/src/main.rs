use server::prelude::*;

use server::data::mempool::Node;

#[tokio::main]
async fn main() -> Result<()> {
    let nodes = Node::get().await?;
    println!("{:#?}", nodes);
    Ok(())
}
