use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::Message;
use teloxide::requests::ResponseResult;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Comandos disponibles:")]
pub enum Command {
    #[command(description = "muestra este mensaje de ayuda")]
    Help,
    #[command(description = "saluda al usuario")]
    Hello,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                Command::descriptions().to_string()
            ).await?;
        }
        Command::Hello => {
            let nombre = msg.from()
                .map(|user| user.first_name.clone())
                .unwrap_or_else(|| "Usuario".to_string());
                
            bot.send_message(
                msg.chat.id,
                format!("¡Hola {}! ¿Cómo estás?", nombre)
            ).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teloxide::types::{Chat, ChatId, MessageId, ChatKind, MessageKind, MessageCommon};

    fn create_test_message(id: i64) -> Message {
        let chat = Chat {
            id: ChatId(id),
            kind: ChatKind::Private(teloxide::types::ChatPrivate {
                first_name: Some("TestUser".to_string()),
                last_name: None,
                username: None,
                bio: None,
                has_private_forwards: None,
                has_restricted_voice_and_video_messages: None,
                emoji_status_custom_emoji_id: None,
            }),
            photo: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: false,
            has_aggressive_anti_spam_enabled: false,
        };

        Message {
            id: MessageId(1),
            date: chrono::Utc::now(),
            chat,
            via_bot: None,
            kind: MessageKind::Common(MessageCommon {
                from: None,
                sender_chat: None,
                forward: None,
                reply_to_message: None,
                edit_date: None,
                author_signature: None,
                media_kind: teloxide::types::MediaKind::Text(teloxide::types::MediaText {
                    text: "".into(),
                    entities: vec![],
                }),
                reply_markup: None,
                is_topic_message: false,
                is_automatic_forward: false,
                has_protected_content: false,
            }),
            thread_id: None,
        }
    }

    #[tokio::test]
    async fn test_answer_hello() {
        let message = create_test_message(1);
        let result = answer(Bot::new("test_token"), message, Command::Hello).await;
        assert!(result.is_err(), "El comando Hello debería fallar con un token inválido");
        if let Err(err) = result {
            assert!(
                err.to_string().contains("Not Found") || 
                err.to_string().contains("error sending request"),
                "Error inesperado: {}", err
            );
        }
    }

    #[tokio::test]
    async fn test_answer_help() {
        let message = create_test_message(2);
        let result = answer(Bot::new("test_token"), message, Command::Help).await;
        assert!(result.is_err(), "El comando Help debería fallar con un token inválido");
        if let Err(err) = result {
            assert!(
                err.to_string().contains("Not Found") || 
                err.to_string().contains("error sending request"),
                "Error inesperado: {}", err
            );
        }
    }
} 