
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::core::command::Command;


pub struct TestCommand;


#[async_trait]
impl Command for TestCommand {

    fn name(self: &Self) -> String {
        "Test".to_string()
    }

    fn short_name(self: &Self) -> String {
        "test".to_string()
    }

    fn description(self: &Self) -> String {
        "A simple test command".to_string()
    }

    async fn run(&self, 
                 context: &Context, 
                 command: &ApplicationCommandInteraction) 
        -> Result<(), String> { 

        let channel = command.channel_id;

        if let Err(error) = channel.send_message(&context.http, |message| message.content("Hey!")).await {
            return Err(format!("{}", error));
        }

        Ok(())
    }
    
}