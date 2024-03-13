// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use serenity::all::Mentionable;
use tokio::time::{Duration, Instant};
use tracing::error;

use crate::{
    database::{self, queries},
    integrations::{self, lastfm::API_AUTH_URL},
    utils::builders,
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Integrations",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Link your Last.fm account.
pub(super) async fn lastfm(ctx: Context<'_>) -> Throwable<()> {
    let db = &ctx.data().db;

    let author = ctx.author();
    let author_id = author.id;

    let lastfm = database::checks::check_lastfm(db, author).await?;
    if lastfm {
        let reply =
            builders::replies::warn_reply_embed("Your Last.fm account is already linked!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let bot = ctx.http().get_current_user().await?;
    let bot_mention = bot.mention();

    let mut message = format!("Linking your Last.fm account...");

    let reply = builders::replies::reply_embed(message, true);

    let reply_handle = ctx.send(reply).await?;

    let get_token = integrations::lastfm::auth::get_token().await?;
    let token = &format!(
        "{}",
        get_token["token"]
            .as_str()
            .expect("auth.token is not a string")
    );

    let res = integrations::lastfm::get(API_AUTH_URL, format!("{token}")).await?;
    let res_url = format!("{}", res.url());
    let res_status = res.status();

    if res_status.is_success() {
        message = format!(
            "Click [here]({res_url}) to grant {bot_mention} permission to use your Last.fm account.\n\n**Note:** You have an hour to grant permission.",
        );

        let reply = builders::replies::reply_embed(message, true);

        reply_handle.edit(ctx, reply).await?;

        let token = Arc::new(token.clone());

        let start_time = Instant::now();

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;

                let elapsed_time = start_time.elapsed();
                if elapsed_time > Duration::from_secs(3600) {
                    break Err("Took too long to grant permission.");
                }

                let session_res =
                    match integrations::lastfm::auth::get_session(format!("{token}")).await {
                        Ok(session_res) => session_res,
                        Err(why) => {
                            error!("Failed to send GET request: {why:?}");
                            break Err("An error occurred while sending a GET request.");
                        }
                    };

                let error = &session_res["error"];
                let error_number = error.as_u64();
                if let Some(number) = error_number {
                    match number {
                        14 => {
                            // Unauthorized Token
                            continue;
                        }
                        _ => {
                            break Err("An error occurred while getting a Last.fm session.");
                        }
                    }
                }

                let session = &session_res["session"];
                let session_name = format!(
                    "{}",
                    session["name"]
                        .as_str()
                        .expect("session.name is not a string")
                );
                let session_key = format!(
                    "{}",
                    session["key"]
                        .as_str()
                        .expect("session.key is not a string")
                );
                break Ok((session_name, session_key));
            }
        });

        let session = match handle.await? {
            Ok(session) => session,
            Err(why) => {
                let reply = builders::replies::error_reply_embed(why, true);

                reply_handle.edit(ctx, reply).await?;

                return Ok(());
            }
        };
        let session_name = &session.0;
        let session_key = &session.1;

        queries::users::update_lastfm_name(db, &author_id, session_name).await?;
        queries::users::update_lastfm_key(db, &author_id, session_key).await?;

        message = format!("Your Last.fm account has been linked.");

        let reply = builders::replies::ok_reply_embed(message, true);

        reply_handle.edit(ctx, reply).await?;
    }

    Ok(())
}
