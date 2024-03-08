// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::{utils::components, Context, Throwable};

#[poise::command(
    slash_command,
    rename = "8ball",
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Ask Magic 8 Ball a question.
pub(super) async fn eightball(
    ctx: Context<'_>,
    #[description = "The question to ask."]
    #[min_length = 3]
    #[max_length = 255]
    question: String,
) -> Throwable<()> {
    let mut rng = StdRng::from_entropy();

    let question = question.to_lowercase();
    if !question.starts_with("who")
        && !question.starts_with("what")
        && !question.starts_with("when")
        && !question.starts_with("where")
        && !question.starts_with("why")
    {
        let reply = components::replies::error_reply_embed(
            format!("\"{question}\" is not an open-ended question!"),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let answers = [positive_answers(), neutral_answers(), negative_answers()];

    let answer = answers
        .choose(&mut rng)
        .and_then(|answers| answers.choose(&mut rng))
        .unwrap();

    let reply = components::replies::reply_embed(format!("🎱 {answer}."), true);

    ctx.send(reply).await?;

    Ok(())
}

fn positive_answers() -> Vec<&'static str> {
    vec![
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
    ]
}

fn neutral_answers() -> Vec<&'static str> {
    vec![
        "Reply hazy, try again",
        "Ask again later",
        "Better not tell you now",
        "Cannot predict now",
        "Concentrate and ask again",
    ]
}

fn negative_answers() -> Vec<&'static str> {
    vec![
        "Don't count on it",
        "My reply is no",
        "My sources say no",
        "Outlook not so good",
        "Very doubtful",
    ]
}
