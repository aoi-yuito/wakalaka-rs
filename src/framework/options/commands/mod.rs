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
mod developer;
mod fun;
mod info;
mod manager;
mod misc;
mod moderator;

use poise::Command;

use crate::{Data, Error};

pub async fn commands() -> Vec<Command<Data, Error>> {
    let mut commands = vec![];
    commands.append(&mut core_commands());
    commands.append(&mut developer_commands());
    commands.append(&mut fun_commands());
    commands.append(&mut info_commands());
    commands.append(&mut manager_commands());
    commands.append(&mut misc_commands());
    commands.append(&mut moderator_commands());
    commands
}

fn moderator_commands() -> Vec<Command<Data, Error>> {
    vec![
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

fn fun_commands() -> Vec<Command<Data, Error>> {
    vec![
        fun::eightball::eightball(),
        fun::flip::flip(),
        fun::hug::hug(),
        fun::roll::roll(),
    ]
}

fn misc_commands() -> Vec<Command<Data, Error>> {
    vec![
        misc::colour::colour(),
        misc::avatar::avatar(),
        misc::banner::banner(),
        misc::suggest::suggest(),
    ]
}

fn manager_commands() -> Vec<Command<Data, Error>> {
    vec![
        manager::emoji::emoji(),
        manager::nick::nick(),
        manager::purge::purge(),
        manager::roles::roles(),
        manager::slowmode::slowmode(),
    ]
}

fn info_commands() -> Vec<Command<Data, Error>> {
    vec![
        info::lookup::lookup(),
        info::info::info(),
        info::invite::invite(),
        info::ping::ping(),
    ]
}

fn developer_commands() -> Vec<Command<Data, Error>> {
    vec![developer::sql::sql()]
}

fn core_commands() -> Vec<Command<Data, Error>> {
    vec![
        core::restrict::restrict(),
        core::setup::setup(),
        core::unrestrict::unrestrict(),
        core::register::register(),
        core::restart::restart(),
        core::shutdown::shutdown(),
        core::unregister::unregister(),
    ]
}
