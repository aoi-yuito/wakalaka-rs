// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::CreateButton;

use crate::utils::builders::{
    self,
    buttons::{
        BUTTON_PAGINATE_FIRST, BUTTON_PAGINATE_LAST, BUTTON_PAGINATE_NEXT, BUTTON_PAGINATE_PREVIOUS,
    },
};

async fn paginated_index(component_id: &String, index: usize, count: usize) -> usize {
    match component_id.as_str() {
        BUTTON_PAGINATE_FIRST => 0,
        BUTTON_PAGINATE_PREVIOUS => {
            if index > 0 {
                index - 1
            } else {
                count - 1
            }
        }
        BUTTON_PAGINATE_NEXT => {
            if index < count - 1 {
                index + 1
            } else {
                0
            }
        }
        BUTTON_PAGINATE_LAST => count - 1,
        _ => index,
    }
}

async fn paginated_buttons(index: usize, count: usize) -> Vec<CreateButton> {
    let buttons = if count > 1 {
        builders::buttons::pagination_buttons((true, true, true, true))
    } else {
        builders::buttons::pagination_buttons((
            index == 0,
            index == 0,
            index == count - 1,
            index == count - 1,
        ))
    };
    buttons
}
