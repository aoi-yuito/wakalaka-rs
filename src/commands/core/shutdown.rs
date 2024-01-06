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

// pub async fn run

use serenity::builder::CreateCommand;

use tracing::log::info;

use crate::Context;

pub async fn run(ctx: &Context) -> Option<String> {
    let seconds = 1;

    let application_name = ctx.http.get_current_application_info().await.unwrap().name;
    info!("Shutting down @{application_name} in {seconds} second(s)...");

    tokio::spawn(async move {
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        std::process::exit(0);
    });

    Some(format!("Shutting down in {seconds} second(s)..."))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("shutdown").description("Shuts down yours truly.")
}
