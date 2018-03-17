#![feature(inclusive_range, inclusive_range_fields, range_contains)]

extern crate dotenv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate serenity;

use std::collections::BTreeMap;
use std::env;
use std::ops::RangeInclusive;

use dotenv::dotenv;
use itertools::Itertools;
use rand::Rng;
use serenity::prelude::*;
use serenity::model::prelude::*;

trait Command {
    fn required_arg_count(&self) -> RangeInclusive<usize>; //impl Range<usize>;
    fn exec(&self, ctx: &Context, msg: &Message, args: &[&str]);
}
type CommandMap = BTreeMap<&'static str, Box<Command + Sync>>;

lazy_static! {
    static ref COMMAND_MAP: CommandMap = {
        let mut commands = CommandMap::new();
        commands.insert("help", Box::new(Help));
        commands.insert("flip", Box::new(Flip));
        commands.insert("roll", Box::new(Roll));
        commands
    };
}

fn main() {
    dotenv().ok();

    let creds = env::var("Z_CREDENTIALS")
        .expect("Unspecified credentials: set Z_CREDENTIALS in the environment (or in `.env`).");
    let mut client = Client::new(&creds, Handler).unwrap();
    client.start().unwrap();
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(Some(Game::playing("Rust")), OnlineStatus::Online)
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().next() == Some('!') {
            let mut tokens = msg.content[1..].split_whitespace();
            let maybe_command = tokens.next();
            let args = tokens.collect::<Vec<_>>();

            if let Some(command) = maybe_command {
                if let Some(command_handler) = COMMAND_MAP.get(command) {
                    let valid_arg_range = command_handler.required_arg_count();
                    if valid_arg_range.contains(args.len()) {
                        command_handler.exec(&ctx, &msg, &*args);
                    } else {
                        msg.reply(&format!(
                            "This command requires {} to {} arguments.",
                            valid_arg_range.start, valid_arg_range.end
                        )).unwrap();
                    }
                } else {
                    msg.reply(&format!("Unknown command: {}", command)).unwrap();
                }
            } else {
                msg.reply("No command specified.").unwrap();
            }
        }
    }
}

struct Help;
impl Command for Help {
    fn required_arg_count(&self) -> RangeInclusive<usize> {
        0..=0
    }

    fn exec(&self, _ctx: &Context, msg: &Message, _args: &[&str]) {
        let help_output = COMMAND_MAP
            .keys()
            .intersperse(&", !")
            .fold("Available commands:\n!".to_owned(), |output, command| {
                output + command
            });

        msg.channel_id.say(&help_output).unwrap();
    }
}

struct Flip;
impl Command for Flip {
    fn required_arg_count(&self) -> RangeInclusive<usize> {
        0..=0
    }

    fn exec(&self, _ctx: &Context, msg: &Message, _args: &[&str]) {
        if rand::thread_rng().gen() {
            msg.reply("Heads!").unwrap();
        } else {
            msg.reply("Tails!").unwrap();
        }
    }
}

struct Roll;
impl Command for Roll {
    fn required_arg_count(&self) -> RangeInclusive<usize> {
        0..=1
    }

    fn exec(&self, _ctx: &Context, msg: &Message, args: &[&str]) {
        let upper_bound = if args.len() == 0 {
            6
        } else if args.len() == 1 {
            if let Ok(num) = args[0].parse() {
                num
            } else {
                msg.reply("That is not a number.").unwrap();
                return;
            }
        } else {
            unreachable!();
        };

        if upper_bound <= 0 {
            msg.reply("The upper bound must be a positive number.")
                .unwrap();
        } else {
            let result = rand::thread_rng().gen_range(0, upper_bound) + 1;
            msg.reply(&format!("{}", result)).unwrap();
        }
    }
}
