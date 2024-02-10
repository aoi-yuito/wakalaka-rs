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

pub mod info;
pub mod invite;
pub mod lookup;
pub mod ping;

const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const CARGO_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const CARGO_RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

const GITHUB_URL: &str = "https://github.com/Kawaxte";

const BOT_INVITE_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=9899241204854&scope=bot";
