use cursive::views::{TextView, Dialog};
use cursive::{Cursive, CursiveExt};

//This is Preston's test main.rs

fn main(){
    let mut siv = Cursive::new();

    siv.add_layer(TextView::new("Welcome to the BlackBoard TUI!\nPress p to quit."));

    siv.add_global_callback('p', |s| s.quit());
    
    siv.run();
}
