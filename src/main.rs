use std::time::Duration;
use crate::person::*;

pub mod person;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, time::TimePlugin};

fn hello_world() {
    println!("hello world!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(queries::GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (commands::add_people, hello_world));
        app.add_systems(Update, (queries::update_people, queries::greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(TimePlugin)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0/60.0)))
        .add_plugins(HelloPlugin)
        .run();
}
