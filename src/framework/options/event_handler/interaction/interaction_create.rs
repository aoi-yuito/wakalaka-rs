// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Interaction;
use tracing::error;

use crate::{
    utils::builders::buttons::{
        BUTTON_PAGINATE_FIRST, BUTTON_PAGINATE_LAST, BUTTON_PAGINATE_NEXT, BUTTON_PAGINATE_PREVIOUS,
    },
    SContext, Throwable,
};

pub(crate) async fn handle(_ctx: &SContext, interaction: &Interaction) -> Throwable<()> {
    match interaction {
        Interaction::Component(interaction) => {
            let component_id = &interaction.data.custom_id;

            match component_id.as_str() {
                BUTTON_PAGINATE_FIRST
                | BUTTON_PAGINATE_PREVIOUS
                | BUTTON_PAGINATE_NEXT
                | BUTTON_PAGINATE_LAST => {
                    // How the fuck am I supposed to move it all here ??
                    todo!();
                }
                _ => {
                    error!("Unhandled component: {component_id}");
                    return Ok(());
                }
            }
        }
        _ => {
            return Ok(());
        }
    }
}
