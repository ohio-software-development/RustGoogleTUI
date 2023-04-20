use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::{event, menu, Cursive, CursiveExt, View};
use cursive_extras::*;
use std::fs;
use std::process::Command;
mod image_view;

fn main() {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("Python3")
            .arg("gmailCleanAPI.py")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
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
        .add_leaf("Calendar", calendar)
        .add_leaf("Logout", |s| s.quit());

    // Show the main dialog box
    let _main_menu = Dialog::around(layout).title("GoogleTUI");

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
    siv.pop_layer(); //Getting rid of previous layer

    let recent_email =
        fs::read_to_string("description.txt").expect("Should have been able to read the file");
    let until = "APIMAIL#1"; //Email 1 indicator
    let positionPlace = match recent_email.find(until) {
        //Finds position of APIMAIL#1 in recent_email string
        Some(pos) => pos,
        None => recent_email.len(),
    };

    let out = &recent_email[0..positionPlace]; //out is email file from beginning to APIMAIL#1

    //Counting how many emails there are in description.txt
    let mut counter = 0;
    let mut isMail = true;
    while isMail {
        let untilNum = format!("{}{}", "APIMAIL#", counter);
        let positionOfAPIMAIL = match recent_email.find(&untilNum) {
            Some(pos) => pos,
            None => recent_email.len(),
        };
        if positionOfAPIMAIL != recent_email.len() {
            counter += 1;
        } else {
            isMail = false;
        }
    }

    let mut select = SelectView::new().autojump();

    for i in 0..counter {
        let show = i + 1;
        select.add_item(show.to_string(), i);
    }

    select.set_on_submit(go_to_next_email);

    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    let emailCount = format!("{}{}{}", "You have ", counter, " emails.");
    // load most recent email
    let layout = LinearLayout::vertical()
        .child(TextView::new("Most Recent Email:"))
        .child(TextView::new(out)) //Output most recent email
        .child(Button::new("Back", go_back_to_main_dialog)) //Go back to main menu
        .child(TextView::new(emailCount))
        .child(
            Dialog::around(select.scrollable().fixed_size((20, 10)))
                .title("Select which email you want to view"),
        );
    siv.add_layer(layout);
}

fn go_to_next_email(siv: &mut Cursive, num: &i32) {
    siv.pop_layer(); //Getting rid of previous layer

    let numNew = *num;
    if numNew == 0 {
        let recent_email =
            fs::read_to_string("description.txt").expect("Should have been able to read the file");
        let mut hold = format!("{}{}", "APIMAIL#", numNew + 1);
        let untilNum = hold.as_str();
        let positionPlace = match recent_email.find(untilNum) {
            //Finds position of APIMAIL#1 in recent_email string
            Some(pos) => pos,
            None => recent_email.len(),
        };

        let out = &recent_email[0..positionPlace]; //out is email file from beginning to APIMAIL#1

        let showEmail = Dialog::new()
            .content(TextView::new(out))
            .button("Back", gmail);
        siv.add_layer(showEmail);
    } else {
        let recent_email =
            fs::read_to_string("description.txt").expect("Should have been able to read the file");
        let mut hold1 = format!("{}{}", "APIMAIL#", numNew);
        let untilNum1 = hold1.as_str();

        let positionPlace1 = match recent_email.find(untilNum1) {
            //Finds position of APIMAIL#1 in recent_email string
            Some(pos) => pos,
            None => recent_email.len(),
        };

        let mut hold2 = format!("{}{}", "APIMAIL#", numNew + 1);
        let untilNum2 = hold2.as_str();
        let positionPlace2 = match recent_email.find(untilNum2) {
            //Finds position of APIMAIL#1 in recent_email string
            Some(pos) => pos,
            None => recent_email.len(),
        };

        let out = &recent_email[positionPlace1..positionPlace2]; //out is email file from beginning to APIMAIL#1

        let showEmail = Dialog::new()
            .content(TextView::new(out))
            .button("Back", gmail);
        siv.add_layer(showEmail);
    }
}

fn calendar(siv: &mut Cursive) {
    // Reads the information in calander.txt

    let file_text = fs::read_to_string("calendar.txt").expect("calendar.txt not read");

    // Text that is left to be searched
    let mut text_left = &file_text[0..file_text.len()];

    // text that is left
    let mut bar_index_option = text_left.find("|");
    let mut bar_index = 1;

    let mut going = true;
    while (going) {
        // get day
        match bar_index_option {
            Some(x) => bar_index = x,
            None => siv.quit(),
        }

        let day = &text_left[0..bar_index];

        // get title
        text_left = &text_left[bar_index + 1..text_left.len()];
        bar_index_option = text_left.find("|");

        match bar_index_option {
            Some(x) => bar_index = x,
            None => siv.quit(),
        }

        let title = &text_left[0..bar_index];

        // get start
        text_left = &text_left[bar_index + 1..text_left.len()];
        bar_index_option = text_left.find("|");
        match bar_index_option {
            Some(x) => bar_index = x,
            None => siv.quit(),
        }

        let start = &text_left[0..bar_index];

        // check if more events exit
        if bar_index + 1 >= text_left.len() {
            going = false;
        } else {
            text_left = &text_left[bar_index + 2..text_left.len()];
        }

        println!("{text_left}");
    }
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
        .child(
            EditView::new()
                .on_submit(search_now)
                .content(search_message),
        );
    // Remove the subdialog box
    siv.pop_layer();

    // Show the main dialog box
    let gmail_layer = Dialog::around(layout).button("Back", go_back_to_main_dialog);
    siv.add_layer(gmail_layer);
}
fn search_now(_: &mut Cursive, message: &str) {
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
