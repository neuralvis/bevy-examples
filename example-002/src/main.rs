// adding a startup system

use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(startup)
        .add_system(normal)
        .run();
}

fn startup(){
    println!("This is a startup system")
}
fn normal() {
    println!("This is a normal system");
}
