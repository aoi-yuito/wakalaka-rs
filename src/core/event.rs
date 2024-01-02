use crate::util::uses::*;

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, crate::Error>,
    _data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            let user_name = data_about_bot.user.name.clone();
            println!("Logged in as '{user_name}'");
        }
        FullEvent::Message { new_message } => {
            let attachments = &new_message.attachments;
            if !attachments.is_empty() {
                let mut metadata = HashMap::new();
                for (index, attachment) in attachments.iter().enumerate() {
                    FileMetadata::read(index, attachment, &mut metadata)
                        .await
                        .expect("Failed to read metadata");
                }

                println!("{:?}", metadata);
            }
        }
        _ => {}
    }
    Ok(())
}
