use cursive::views::{TextView, Dialog};
use cursive::{Cursive, CursiveExt};


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Blackboard Rust TUI"))
        .title("Menu")
        .button("Login", |s| s.quit())
        .button("Team Members", |s| s.add_layer(
            Dialog::around(TextView::new("Brady Phelps\nMichael Tan\nnPreston Rembis\nAlex Bikowski"))
            .button("Back", |s| s.quit())
        
        ))
        .button("How to Use", |s| s.add_layer(

            Dialog::info("change buttons using the arrow keys and press enter to click a button. q quits the TUI. Additional buttons to quit like the one bellow will appear. buttons can also be clicked using your mouse.")
            .button("Quit", |s| s.quit())

        ))
        .button("Quit", |s| s.quit())
                        
    );

    siv.add_global_callback('q', |s| s.quit());

    // Starts the event loop.
    siv.run();
}

