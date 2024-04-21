// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rand::{prelude::SliceRandom, rngs::StdRng, SeedableRng};
use serenity::all::Mentionable;
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    rename = "8ball",
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Ask Magic 8-Ball a question.
pub(super) async fn eightball(
    ctx: Context<'_>,
    #[description = "Question to ask."]
    #[min_length = 3]
    #[max_length = 255]
    question: String,
) -> Throwable<()> {
    let mut rng = StdRng::from_entropy();

    let result = if !is_question_open_ended(&question) {
        Err(format!("{question:?} is not an open-ended question."))
    } else {
        let answers = gather_all_answers();

        let answer = answers
            .choose(&mut rng)
            .and_then(|answers| Some(format!("{answers}")))
            .expect("No answers found.");

        let author = ctx.author();
        let author_mention = author.mention();

        Ok(format!(":8ball: {answer}, {author_mention}."))
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_reply_with_embed(msg, false),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, false),
    };

    ctx.send(reply).await?;

    Ok(())
}

fn is_question_open_ended(question: impl Into<String>) -> bool {
    let question = question.into().trim().to_lowercase();
    question.starts_with("who")
        || question.starts_with("what")
        || question.starts_with("when")
        || question.starts_with("where")
        || question.starts_with("why") && question.ends_with("?")
}

fn gather_all_answers() -> Vec<&'static str> {
    let positive_answers = vec![
        "It is certain",
        "It is decidedly so",
        "Without a doubt",
        "Yes definitely",
        "You may rely on it",
        "As I see it, yes",
        "Most likely",
        "Outlook good",
        "Yes",
        "Signs point to yes",
    ];
    let neutral_answers = vec![
        "Reply hazy, try again",
        "Ask again later",
        "Better not tell you now",
        "Cannot predict now",
        "Concentrate and ask again",
    ];
    let negative_answers = vec![
        "Don't count on it",
        "My reply is no",
        "My sources say no",
        "Outlook not so good",
        "Very doubtful",
    ];

    [positive_answers, neutral_answers, negative_answers].concat()
}
