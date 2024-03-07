// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::info;

use crate::Error;

pub(crate) async fn handle(shard_count: &u32) -> Result<(), Error> {
    if *shard_count == 1 {
        info!("Readied 1 shard");
    } else {
        info!("Readied {shard_count} shards");
    }

    Ok(())
}
