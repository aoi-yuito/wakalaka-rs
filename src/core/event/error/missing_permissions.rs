/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub async fn on_missing_bot_permissions(missing_permissions: Permissions, ctx: crate::Context<'_>) {
    ctx.reply(&format!(
        "I'm missing the following permission(s): `{missing_permissions:?}`"
    ))
    .await
    .unwrap();
}

pub async fn on_missing_user_permissions(
    missing_permissions: Option<Permissions>,
    ctx: crate::Context<'_>,
) {
    ctx.reply(&format!(
        "You're missing the following permission(s): `{missing_permissions:?}`"
    ))
    .await
    .unwrap();
}
