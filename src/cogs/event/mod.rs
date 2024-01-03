pub mod error;
pub mod message;
pub mod reaction_add;
mod ready;

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
use crate::uses::*;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, crate::Data, crate::Error>,
    _data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            ready::ready(data_about_bot).await?;
        }
        FullEvent::Message { new_message } => {
            message::on_message(ctx, new_message).await?;
        }
        FullEvent::ReactionAdd { add_reaction } => {
            reaction_add::on_reaction_add(ctx, add_reaction.clone()).await?;
        }
        _ => {}
    }
    Ok(())
}
