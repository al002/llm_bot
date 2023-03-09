use teloxide::{prelude::*, utils::command::BotCommands, types::Me };

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
pub enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "shows your ID.")]
    MyId,
}

pub async fn commands_handler(
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        SimpleCommand::Help => {
            SimpleCommand::descriptions().to_string()
        }
        SimpleCommand::MyId => {
            format!("{}", msg.from().unwrap().id)
        }
    };

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn group_message_handler(
    _bot: Bot,
    me: Me,
    msg: Message,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");
    if text.contains(&me.mention()) {
        println!("mention");
    } else {
        println!("not mention");
    }
    Ok(())
}

pub async fn message_handler(bot: Bot, me: Me, msg: Message) -> Result<(), teloxide::RequestError> {
    bot.send_message(msg.chat.id, me.username()).await?;
    Ok(())
}
