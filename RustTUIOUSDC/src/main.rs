use cursive::views::{TextView, Dialog};
use cursive::{Cursive, CursiveExt};


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    let _main_menu = Dialog::new()
        .title("Menu")
        .content(TextView::new("Blackboard Rust TUI"))
        .button("Login", |s| s.quit())
        .button("Team Members", open_subdialog)
        .button("Quit", |s| s.quit());

    siv.add_layer(_main_menu);

    siv.add_global_callback('q', |s| s.quit());
    

    siv.run();
}

fn open_subdialog(siv: &mut Cursive)
{
    siv.pop_layer();

    siv.add_layer(
        Dialog::new()
            .title("Team Members")
            .content(TextView::new("Brady Phelps\nMichael Tan\nPreston Rembis\nAlex Bikowski"))
            .button("Back", go_back_to_main_dialog),
    );
}

fn go_back_to_main_dialog(siv: &mut Cursive) {
    // Remove the subdialog box
    siv.pop_layer();

    // Show the main dialog box
    siv.add_layer(
        Dialog::new()
            .title("Menu")
            .content(TextView::new("Blackboard Rust TUI"))
            .button("Login", |s| s.quit())
            .button("Team Members", open_subdialog)
            .button("Quit", |s| s.quit())
    );
}
