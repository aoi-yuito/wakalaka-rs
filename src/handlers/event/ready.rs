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

use poise::serenity_prelude::{ActivityData, OnlineStatus};

use crate::serenity::Context;
use crate::serenity::Ready;

pub(super) fn handle(_: &Ready, ctx: &Context) {
    ctx.set_presence(
        Some(ActivityData::listening("Dubidubidu")),
        OnlineStatus::Online,
    );
}
