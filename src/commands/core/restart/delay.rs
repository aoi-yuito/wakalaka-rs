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

use serenity::all::ResolvedValue;
use serenity::model::application::ResolvedOption;

pub(super) fn delay(options: &[ResolvedOption<'_>]) -> Option<String> {
    let seconds = options
        .get(1)
        .and_then(|opt| {
            match &opt.value {
                ResolvedValue::Integer(i) => Some(*i),
                _ => None,
            }
        })
        .unwrap_or(5);
    if seconds < 5 {
        return Some("Delay cannot be less than 5 seconds.".to_string());
    } else if seconds > 60 {
        return Some("Delay cannot be more than 60 seconds (1 minute).".to_string());
    }
    None
}
