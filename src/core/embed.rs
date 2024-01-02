// use crate::util::uses::*;

// pub fn is_embed_containing_image(msg: &Message) -> bool {
//     for embed in &msg.embeds {
//         if embed.image.is_some() || embed.thumbnail.is_some() {
//             return true;
//         }
//     }
//     false
// }

// pub fn get_embed_urls(msg: &Message) -> Vec<String> {
//     let mut urls = Vec::new();

//     for embed in &msg.embeds {
//         if let Some(image) = &embed.image {
//             urls.push(image.url.clone());
//         }
//         if let Some(thumbnail) = &embed.thumbnail {
//             urls.push(thumbnail.url.clone());
//         }
//     }
//     urls
// }
