use cursive::views::{TextView, Dialog};
use cursive::{Cursive, CursiveExt};


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Blackboard Rust TUI"))
        .title("Menu")
        
        // button where login will occur
        .button("Login", |s| s.quit())

        // button to display team members
        .button("Team Members", |s| s.add_layer(
            Dialog::info("Brady Phelps\nMichael Tan\nnPreston Rembis\nAlex Bikowski")
        ))

        // button to exit TUI
        .button("Quit", |s| s.quit())
                        
    );

    // q will always quit the TUI
    siv.add_global_callback('q', |s| s.quit());

    // Starts the event loop.
    siv.run();
}

