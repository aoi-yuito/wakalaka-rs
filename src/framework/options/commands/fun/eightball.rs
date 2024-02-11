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

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::{utility::components::messages, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    rename = "8ball",
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
// Ask the Magic 8 Ball a question.
pub async fn eightball(
    ctx: Context<'_>,
    #[description = "The question to ask."]
    #[min_length = 2]
    #[max_length = 120]
    question: String,
) -> Result<(), Error> {
    let mut rng = StdRng::from_entropy();

    let lc_question = question.to_lowercase();
    if !lc_question.contains("who")
        || !lc_question.contains("what")
        || !lc_question.contains("when")
        || !lc_question.contains("where")
        || !lc_question.contains("why")
    {
        let reply = messages::error_reply(
            "Magic 8 Ball can only answer `yes` or `no` questions!",
            false,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let answers = [positive_answers(), neutral_answers(), negative_answers()];

    let answer = answers
        .choose(&mut rng)
        .and_then(|answers| answers.choose(&mut rng))
        .expect("Failed to find an answer.");
    let reply = messages::reply(format!("🎱 {answer}"), false);
    ctx.send(reply).await?;

    Ok(())
}

fn negative_answers() -> Vec<&'static str> {
    vec![
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ]
}

fn neutral_answers() -> Vec<&'static str> {
    vec![
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
    ]
}

fn positive_answers() -> Vec<&'static str> {
    vec![
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
    ]
}
