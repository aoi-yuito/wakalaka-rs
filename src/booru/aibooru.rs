use crate::util::uses::*;

#[poise::command(slash_command, subcommands("post"), category = "Booru")]
pub async fn aibooru(_: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn post(
    ctx: crate::Context<'_>,
    #[description = "The ID of the post."] id: i64,
) -> Result<(), crate::Error> {
    let mut _reply = CreateReply {
        content: None,
        ..Default::default()
    };

    // Posts below 0, or at the ID of 0, don't exist on the website.
    let exists = Post::exists(ctx, _reply.clone(), id).await?;
    if exists {
        let posts_show_url = format!("{AIBOORU_URL}/posts/{id}.json");

        let client = reqwest::Client::new();
        let response_text = client.get(&posts_show_url).send().await?.text().await?;
        let response_json = serde_json::from_str(&response_text).unwrap();

        // Posts that don't exist on the website return a JSON object with a "success" key set to false.
        let success = Post::is_success(ctx, &response_json, _reply, id).await?;
        if success {
            let post_data = Post::extract_post_data(&response_json);
            let embed = Post::create_embed(&post_data, id, None, AIBOORU_URL);

            _reply = CreateReply {
                content: None,
                embeds: vec![embed],
                ..Default::default()
            };
            let _ = ctx.send(_reply).await?;
        }
    }

    Ok(())
}
