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
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.\

mod core;
mod fun;
mod info;
mod manager;
mod misc;
mod moderator;

use poise::Command;

use crate::{Data, Error};

// pub(crate) async fn global_commands() -> Vec<Command<Data, Error>> {
//     vec![]
// }

pub(crate) async fn guild_commands() -> Vec<Command<Data, Error>> {
    vec![
        core::restart::restart(),
        core::shutdown::shutdown(),
        fun::flip::flip(),
        fun::hug::hug(),
        fun::roll::roll(),
        info::info::info(),
        info::ping::ping(),
        manager::emoji::emoji(),
        manager::nick::nick(),
        manager::purge::purge(),
        manager::role::role(),
        manager::slowmode::slowmode(),
        misc::colour::colour(),
        misc::avatar::avatar(),
        misc::banner::banner(),
        misc::suggest::suggest(),
        moderator::deafen::deafen(),
        moderator::ban::ban(),
        moderator::kick::kick(),
        moderator::mute::mute(),
        moderator::timeout::timeout(),
        moderator::unban::unban(),
        moderator::undeafen::undeafen(),
        moderator::unmute::unmute(),
        moderator::untimeout::untimeout(),
        moderator::unwarn::unwarn(),
        moderator::warn::warn(),
        moderator::warnings::warnings(),
    ]
}
