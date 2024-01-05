// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

pub mod cache_ready;
pub mod interaction_create;
pub mod ready;

use serenity::{
    all::{GuildId, Interaction, Ready},
    async_trait,
    builder::CreateCommand,
    client::EventHandler,
};

use crate::commands::*;
use crate::Context;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::handle(ctx, interaction).await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        cache_ready::handle(ctx, guilds).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle(ctx, ready).await;
    }
}

fn created_global_commands() -> Vec<CreateCommand> {
    vec![general::avatar::register()]
}

fn created_commands() -> Vec<CreateCommand> {
    vec![core::restart::register()]
}
