use cursive::views::{TextView, Dialog};
use cursive::{Cursive, CursiveExt};


//Testing for Preston_test branch

fn main(){
    let mut siv = Cursive::new();

    siv.add_layer(TextView::new("Welcome to the BlackBoard TUI!\nPress q to quit."));

    siv.add_global_callback('q', |s| s.quit());
    
    siv.run();
}
