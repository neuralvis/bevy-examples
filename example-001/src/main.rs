// adding a startup system

use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(startup)
        .add_system(hello_world_system)
        .run();
}

fn startup(){
    println!("This is a startup system")
}
fn hello_world_system() {
    println!("hello world");
}