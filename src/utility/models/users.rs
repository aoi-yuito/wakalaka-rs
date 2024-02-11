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

use serenity::{all::CurrentUser, model::ModelError};
use tracing::error;

use crate::Context;

pub async fn bot(ctx: Context<'_>) -> Result<CurrentUser, ModelError> {
    match ctx.http().get_current_user().await {
        Ok(current_user) => Ok(current_user),
        Err(why) => {
            error!("Failed to get current user: {why:?}");
            return Err(ModelError::MemberNotFound);
        }
    }
}
