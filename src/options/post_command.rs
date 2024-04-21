// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::info;
use wakalaka_core::types::Context;
use wakalaka_utils::accessors;

pub(super) async fn handle_post_command_option(ctx: Context<'_>) {
    let user = ctx.author();
    let user_name = &user.name;

    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let guild = accessors::guilds::fetch_guild(ctx).expect("Failed to fetch guild");
    let guild_name = &guild.name;

    info!("@{user_name} invoked /{command_qname} in {guild_name}");
}
