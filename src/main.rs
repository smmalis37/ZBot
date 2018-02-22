#![feature(inclusive_range_syntax)]

extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate serenity;

use std::collections::BTreeMap;
use itertools::Itertools;
use rand::Rng;
use serenity::prelude::*;
use serenity::model::prelude::*;

type CommandMap = BTreeMap<&'static str, Command>;
type Command = fn(ctx: &Context, msg: &Message, args: &[&str]);

lazy_static! {
    static ref COMMAND_MAP: CommandMap = {
        let mut commands = CommandMap::new();
        commands.insert("help", help);
        commands.insert("flip", flip);
        commands.insert("roll", roll);
        commands
    };
}

fn main() {
    let creds = "NDE2MDA0Nzc4NTg2NDA2OTEy.DW-KXw.G6PG3xN2EvrukNilq4iSdPmx2bU";
    let mut client = Client::new(creds, Handler).unwrap();
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
                    command_handler(&ctx, &msg, &*args);
                } else {
                    msg.reply(&format!("Unknown command: {}", command)).unwrap();
                }
            } else {
                msg.reply("No command specified.").unwrap();
            }
        }
    }
}

fn help(_ctx: &Context, msg: &Message, args: &[&str]) {
    if args.len() != 0 {
        msg.reply("This command does not accept any arguments.")
            .unwrap();
        return;
    }

    let help_output = COMMAND_MAP
        .keys()
        .intersperse(&", !")
        .fold("Available commands:\n!".to_owned(), |output, command| {
            output + command
        });

    msg.channel_id.say(&help_output).unwrap();
}

fn flip(_ctx: &Context, msg: &Message, args: &[&str]) {
    if args.len() != 0 {
        msg.reply("This command does not accept any arguments.")
            .unwrap();
        return;
    }

    if rand::thread_rng().gen() {
        msg.reply("Heads!").unwrap();
    } else {
        msg.reply("Tails!").unwrap();
    }
}

fn roll(_ctx: &Context, msg: &Message, args: &[&str]) {
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
        msg.reply("This command only accepts one argument.")
            .unwrap();
        return;
    };

    if upper_bound <= 0 {
        msg.reply("The upper bound must be a positive number.")
            .unwrap();
    } else {
        let result = rand::thread_rng().gen_range(0, upper_bound) + 1;
        msg.reply(&format!("{}", result)).unwrap();
    }
}
