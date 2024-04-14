// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::info;
use wakalaka_core::types::Throwable;

pub(crate) async fn handle_shards_ready_event(shard_count: &u32) -> Throwable<()> {
    if *shard_count == 1 {
        info!("Readied {shard_count} shard");
    } else {
        info!("Readied {shard_count} shards");
    }

    Ok(())
}
