use std::{
    io,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::{Res, Resource},
    },
    log::info,
};
use clap::{Parser, Subcommand};

use core_library::authentication::client_authentication::{
    PasswordLoginInfo, SignInEvent, SignOutEvent, SignUpEvent,
};

pub struct ConsoleParserPlugin;

impl Plugin for ConsoleParserPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (sender, receiver) = mpsc::channel::<String>();
        app.insert_resource(StdInChannel {
            send: sender,
            receive: Mutex::new(receiver),
        });
        app.add_event::<StdInEvents>();

        app.add_systems(Startup, start_std_in_reader);
        app.add_systems(Update, (read_stdin, try_parse_stdin));
    }
}

#[derive(Parser)]
#[command(name = "ags")]
#[command(about = "CLI interface to control an Arts Game Server", long_about = None)]
#[command(no_binary_name = true)]
struct AppCommands {
    #[command(subcommand)]
    commands: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Signs up as a server account using the provided email and password
    SignUp { email: String, password: String },
    /// Signs into as a server account using the provided email and password
    SignIn { email: String, password: String },
    /// Signs out of the app if its currently signed in
    SignOut,
}

#[derive(Resource)]
struct StdInChannel {
    send: Sender<String>,
    receive: Mutex<Receiver<String>>,
}

fn read_stdin(mut eventwriter: EventWriter<StdInEvents>, channel: Res<StdInChannel>) {
    if let Ok(channel) = channel.receive.try_lock() {
        if let Ok(message) = channel.try_recv() {
            eventwriter.send(StdInEvents { line: message })
        }
    }
}

fn start_std_in_reader(channel: Res<StdInChannel>) {
    let sender = channel.send.clone();
    let stdin = io::stdin(); // We get `Stdin` here.
    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            loop {
                let mut buffer = String::new();
                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        let _ = sender.send(buffer);
                    }
                    Err(_) => {
                        info!("error reading from StdIn");
                        continue;
                    }
                }
            }
        })
        .detach();
}

fn try_parse_stdin(
    mut event_reader: EventReader<StdInEvents>,
    mut sign_in_events: EventWriter<SignInEvent>,
    mut sign_up_events: EventWriter<SignUpEvent>,
    mut sign_out_events: EventWriter<SignOutEvent>,
) {
    for stdin_line in event_reader.read() {
        let mut line = vec![];
        for word in stdin_line.line.trim().split_whitespace() {
            line.push(word);
        }
        match AppCommands::try_parse_from(line) {
            Ok(command) => match command.commands {
                SubCommands::SignUp { email, password } => sign_up_events.send(SignUpEvent {
                    info: PasswordLoginInfo::new(&email, &password, false),
                }),
                SubCommands::SignIn { email, password } => sign_in_events.send(SignInEvent {
                    login_info: PasswordLoginInfo::new(&email, &password, false),
                }),
                SubCommands::SignOut => sign_out_events.send(SignOutEvent),
            },
            Err(err) => info!("Error {}", err),
        }
    }
}

#[derive(Event)]
struct StdInEvents {
    line: String,
}
