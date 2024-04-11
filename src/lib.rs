mod pb;
use crate::pb::sf::cosmos::r#type::v2::Block;
use crate::pb::cosmos::v1::BlockStats;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn block_to_stats(block: Block) -> Result<BlockStats, Error> {
    let mut stats = BlockStats::default();
    let header =  block.header.as_ref().unwrap();
    let last_block_id = header.last_block_id.as_ref().unwrap();

    stats.block_height = block.height as u64;
    stats.block_hash = hex::encode(block.hash);
    stats.block_time = block.time;
    stats.block_proposer = hex::encode(&header.proposer_address);
    stats.parent_hash = hex::encode(&last_block_id.hash);
    stats.parent_height = block.height - 1i64;

    stats.num_txs = block.txs.len() as u64;

    Ok(stats)
}
