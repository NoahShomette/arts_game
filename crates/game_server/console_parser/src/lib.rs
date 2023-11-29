use std::{
    ffi::OsString,
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

use arts_core::authentication::client_authentication::{
    PasswordLoginInfo, SignInEvent, SignUpEvent,
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
struct AppCommands {
    #[command(subcommand)]
    commands: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    SignUp { email: String, password: String },
    SignIn { email: String, password: String },
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
    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            match stdin.read_line(&mut buffer) {
                Ok(_) => {
                    let _ = sender.send(buffer);
                }
                Err(_) => {
                    info!("error reading from StdIn");
                    return;
                }
            }
        })
        .detach();
}

fn try_parse_stdin(
    mut event_reader: EventReader<StdInEvents>,
    mut sign_in_events: EventWriter<SignInEvent>,
    mut sign_up_events: EventWriter<SignUpEvent>,
) {
    for stdin_line in event_reader.read() {
        match AppCommands::try_parse_from(vec![OsString::from(stdin_line.line.clone())].iter()) {
            Ok(command) => match command.commands {
                SubCommands::SignUp { email, password } => sign_up_events.send(SignUpEvent {
                    info: PasswordLoginInfo::new(&email, &password, false),
                }),
                SubCommands::SignIn { email, password } => sign_in_events.send(SignInEvent {
                    login_info: PasswordLoginInfo::new(&email, &password, false),
                }),
            },
            Err(err) => info!("{}", err),
        }
    }
}

#[derive(Event)]
struct StdInEvents {
    line: String,
}
