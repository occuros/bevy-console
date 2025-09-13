use bevy::prelude::*;
use bevy_console::{ConsoleCommandEntered, ConsolePlugin, ConsoleSet};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_systems(Update, raw_commands.in_set(ConsoleSet::Commands))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .run();
}

fn raw_commands(mut console_commands: EventReader<ConsoleCommandEntered>) {
    for ConsoleCommandEntered { command_name, args } in console_commands.read() {
        println!(r#"Entered command "{command_name}" with args {:#?}"#, args);
    }
}
