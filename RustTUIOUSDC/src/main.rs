use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::{event, menu, Cursive, CursiveExt};
use cursive_extras::*;
use std::fs;
use std::process::Command;

mod image_view;

fn main() {
    let mut siv = Cursive::new();

    siv.set_theme(better_theme());

    // notes:
    // .child(EditView::new().content("blahblahblah"));

    // img = image_view::ImageView::set_image(&mut img, "IMG_7223[20].png");
    let _login_menu = Dialog::around(styled_editview("", "Login", true))
        .button("Enter", go_back_to_main_dialog)
        .button("Quit", |view| view.quit())
        .title("Login");
    // image
    // siv.add_layer(layout);
    siv.add_layer(_login_menu);

    siv.set_autohide_menu(false);
    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn go_back_to_main_dialog(siv: &mut Cursive) {
    let mut img = image_view::ImageView::new(40, 14);
    img.set_image("./images/download.jpeg");
    let image_viewer = Dialog::around(img);

    let current_val =
        fs::read_to_string("./description.txt").expect("Should have been able to read the file");

    let layout = LinearLayout::vertical()
        .child(TextView::new("Display:"))
        .child(image_viewer)
        .child(TextView::new("Output:"))
        .child(TextView::new(current_val));

    // Remove the subdialog box
    siv.pop_layer();

    // create the menu bar
    siv.menubar().clear();

    // create a subtree of friends
    let friends = vec![
        "Brady Phelps",
        "Michael Tan",
        "Alex Bikowski",
        "Preston Rembis",
    ];
    let mut friends_tree = menu::Tree::new();
    for friend in friends {
        friends_tree.add_leaf(friend, |s| swap_data(s, friend));
    }

    siv.menubar()
        .add_leaf("Home", go_back_to_main_dialog)
        .add_subtree(
            "Browser",
            menu::Tree::new()
                .leaf("Search", open_search)
                .leaf("Drive", open_drive)
                .leaf("Slides", open_slides)
                .leaf("Sheets", open_sheets),
        )
        .add_subtree("Team", friends_tree)
        .add_leaf("Gmail", gmail)
        .add_leaf("Logout", |s| s.quit());

    // Show the main dialog box
    let _main_menu = Dialog::around(layout).title("MyTui");

    // image
    siv.add_layer(_main_menu);
}

fn swap_data(siv: &mut Cursive, name: &str) {
    siv.pop_layer();
    let file_path = "bios/".to_string() + name + ".bio";
    let bio = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut img = image_view::ImageView::new(40, 14);
    let image_path = String::from("./images/") + name + ".jpeg";
    img.set_image(&image_path);
    let image_viewer = Dialog::around(img);

    let layout = LinearLayout::vertical()
        .child(TextView::new("Profile:"))
        .child(image_viewer)
        .child(TextView::new("Bio:"))
        .child(TextView::new(bio));

    // Show the main dialog box
    let content = Dialog::around(layout)
        .title("MyTui")
        .button("Like", go_back_to_main_dialog)
        .button("Dislike", go_back_to_main_dialog);

    // image
    siv.add_layer(content);
}

fn gmail(siv: &mut Cursive) {
    let recent_email =
        fs::read_to_string("description.txt").expect("Should have been able to read the file");

    // load most recent email

    let output = Command::new("Python3")
        .args(["../gmailCleanAPI.py"])
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;

    let layout = LinearLayout::vertical()
        .child(TextView::new("Gmail:"))
        .child(TextView::new("Display:"))
        .child(EditView::new().on_submit(save_mail).content(recent_email));
    // Remove the subdialog box
    siv.pop_layer();

    // Show the main dialog box
    let gmail_layer = Dialog::around(layout).button("Back", go_back_to_main_dialog);
    siv.add_layer(gmail_layer);
}

fn save_mail(_: &mut Cursive, x: &str) {
    let data = x;
    fs::write("description.txt", data).expect("Unable to write file");
}

fn open_search(siv: &mut Cursive) {
    let search_message = "Type here...";

    let layout = LinearLayout::vertical()
        .child(TextView::new("Gmail:"))
        .child(TextView::new("Display:"))
        .child(EditView::new().on_submit(search_now).content(search_message));
    // Remove the subdialog box
    siv.pop_layer();

    // Show the main dialog box
    let gmail_layer = Dialog::around(layout).button("Back", go_back_to_main_dialog);
    siv.add_layer(gmail_layer);
}
fn search_now (_: &mut Cursive, message: &str){
    let path = "https://www.google.com/search?q=";
    let search_path = format!("{}{}", path, message);
    match open::that(search_path.clone()) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", search_path, err),
    }
}
fn open_drive(_: &mut Cursive) {
    let path = "https://drive.google.com/drive/u/0/my-drive";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}
fn open_sheets(_: &mut Cursive) {
    let path = "https://docs.google.com/spreadsheets/u/0/";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}
fn open_slides(_: &mut Cursive) {
    let path = "https://docs.google.com/presentation/u/0/";
    match open::that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }
}

// todo: series of functions to display different UI menus for social media interface
// todo: image renderer?
// todo: put together presentation and polish idea?? (idk if they using dev post or how we are supposed to submit so idk ab this yet)
