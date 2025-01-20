use teloxide::{
    prelude::*, 
    types::{
        Chat, ChatId, MessageId,
        ChatKind, MessageKind, MessageCommon,
    }
};
use chrono::Utc;

use solana_allstars_ve_bot::{Command, answer};

#[tokio::test]
async fn test_hello_command_response() {
    let chat = Chat {
        id: ChatId(1),
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

    let message = Message {
        id: MessageId(1),
        date: Utc::now(),
        chat: chat,
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
    };

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
async fn test_help_command_response() {
    let chat = Chat {
        id: ChatId(1),
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

    let message = Message {
        id: MessageId(1),
        date: Utc::now(),
        chat: chat,
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
    };

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
