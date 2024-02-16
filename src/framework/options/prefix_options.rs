// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::PrefixFrameworkOptions;

use crate::{Data, Error};

pub(crate) fn prefix_framework_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        prefix: Some(format!("?")),
        mention_as_prefix: false,
        ignore_edits_if_not_yet_responded: true,
        execute_self_messages: false,
        case_insensitive_commands: true,
        ..Default::default()
    }
}
