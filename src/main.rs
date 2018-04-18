extern crate dotenv;
extern crate rand;
extern crate serenity;

use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use rand::Rng;
use serenity::framework::standard::*;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    let _ = dotenv();

    let creds = env::var("Z_CREDENTIALS")
        .expect("Unspecified credentials: set Z_CREDENTIALS in the environment (or in `.env`).");
    let mut client = Client::new(&creds, Handler).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.depth(1).prefix("!"))
            .group("Rng", |g| g.cmd("flip", Flip).cmd("roll", Roll))
            .on_dispatch_error(dispatch_error_handler)
            .unrecognised_command(unrecognised_command_handler)
            .customised_help(help_commands::plain, |h| {
                h.striked_commands_tip(None)
                    .dm_and_guilds_text("Everywhere")
                    .guild_only_text("Only in text channels")
            }),
    );
    client.start().unwrap();
}

fn dispatch_error_handler(_ctx: Context, msg: Message, err: DispatchError) {
    msg.reply(&format!("{:?}", err)).unwrap();
}

fn unrecognised_command_handler(_ctx: &mut Context, msg: &Message, cmd: &str) {
    msg.reply(&format!("{} is not a command.", cmd)).unwrap();
}

struct Flip;
impl Command for Flip {
    fn options(&self) -> Arc<CommandOptions> {
        let mut options = CommandOptions::default();
        options.desc = Some("Flips a coin.".to_owned());
        options.min_args = Some(0);
        options.max_args = Some(0);
        Arc::new(options)
    }

    fn execute(&self, _ctx: &mut Context, msg: &Message, _args: Args) -> Result<(), CommandError> {
        let result = if rand::thread_rng().gen() {
            "Heads!"
        } else {
            "Tails!"
        };

        msg.reply(result).unwrap();
        Ok(())
    }
}

struct Roll;
impl Command for Roll {
    fn options(&self) -> Arc<CommandOptions> {
        let mut options = CommandOptions::default();
        options.desc = Some("Rolls a die.".to_owned());
        options.min_args = Some(0);
        options.max_args = Some(1);
        Arc::new(options)
    }

    fn execute(
        &self,
        _ctx: &mut Context,
        msg: &Message,
        mut args: Args,
    ) -> Result<(), CommandError> {
        let upper_bound = match args.single() {
            Ok(num) => Ok(num),
            Err(ArgError::Eos) => Ok(6),
            Err(ArgError::Parse(err)) => Err(CommandError(format!("{}", err))),
        };

        upper_bound.map(|up| {
            let result = rand::thread_rng().gen_range(0, up) + 1;
            msg.reply(&format!("{}", result)).unwrap();
        })
    }
}
