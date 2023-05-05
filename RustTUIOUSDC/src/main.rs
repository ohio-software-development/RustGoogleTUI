use cursive::event::EventResult;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{Button, Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::{event, menu, Cursive, CursiveExt, View};
use cursive_extras::*;
use std::fs;
use std::process::Command;
use std::thread;
use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;
mod image_view;
use std::path::Path;
use std::env::set_current_dir;

fn main() {
    let rpath = dirs::home_dir().unwrap().display().to_string() + "/RustGoogleTUI/RustTUIOUSDC/src/";
    let current_directory = Path::new(&rpath);
    set_current_dir(current_directory);
    thread::spawn(|| {
        // SWITCH TO PYTHON IF RUNNING REGULAR PYTHON AND NOT PYTHON3 (also change at line 143)
        let output = Command::new("python3")
            .arg("gmailLoader.py")
            .output()
            .expect("failed to execute process");
        let hello = output.stdout;
    });
    let mut siv = Cursive::new();

    siv.set_theme(better_theme());
    
    let mut _title_menu = Dialog::text("Error Screen");
        
    
    let token_found = std::path::Path::new("token.json").exists();
    if token_found{
        _title_menu = Dialog::text("Welcome to the Google TUI Project!")
            .button("Enter", go_back_to_main_dialog);
    }
    else{
        _title_menu = Dialog::text("Welcome to the Google TUI Project!\nLogin in the Web Browser Before Proceeding.")
            .button("Enter", find_token);
    }
    siv.add_layer(_title_menu);
    
    siv.set_autohide_menu(false);
    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn find_token(siv: &mut Cursive){
    siv.pop_layer();
    let token_found = std::path::Path::new("token.json").exists();
    let mut login_check = Dialog::text("Error Screen");
    if token_found{
        login_check = Dialog::text("Successfully Logged In.")
            .button("Enter", go_back_to_main_dialog);
    }
    else{
        login_check = Dialog::text("Invalid Attempt. Please go to Google Login Webpage")
            .button("Go Back",find_token);
            
        thread::spawn(|| {
            let output = Command::new("python3")
                .arg("login_system.py")
                .output()
                .expect("failed to execute process");
        });
    }
    siv.add_layer(login_check);
}

fn go_back_to_main_dialog(siv: &mut Cursive) {
    let mut img = image_view::ImageView::new(40, 14);
    img.set_image("../images/download.jpeg");
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
        .add_subtree(
            "Gmail",
            menu::Tree::new()
                .leaf("Read", gmail)
                .leaf("Send", send_layer),
        )
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
    let image_path = String::from("../images/") + name + ".jpeg";
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

fn mess_layer(siv: &mut Cursive, to: &str, subject: &str)
{

    let output = format!("To: {}\nSubject: {}", to, subject);
    siv.pop_layer();

    //Makes copy of email recipient
    let to_copy = Rc::new(to.to_owned());

    //Clones the copy of the recipient, repeated for the subject
    let to_copy_clone = Rc::clone(&to_copy);

    let sub_copy = Rc::new(subject.to_owned());

    let sub_copy_clone = Rc::clone(&sub_copy);
    siv.add_layer(
        Dialog::new()
            .title("SEND")
            .content(
                EditView::new()
                    .on_submit(move |siv, message| {
                        send(siv, &to_copy_clone, &sub_copy_clone, message);
                        siv.pop_layer();
                        siv.add_layer(Dialog::new()
                            .title("Successfully Sent Email")//Pop up message when email is sent using enter key
                            .button("Back to Main Menu", go_back_to_main_dialog)); 
                    })
                    .with_name("message")
                    .fixed_width(20),
            )
            .button("Send Message", move |siv| {
                let message = siv
                    .call_on_name("message", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                send(siv, &to_copy, &sub_copy, &message);
                siv.pop_layer();
                siv.add_layer(Dialog::new()
                    .title("Successfully Sent Email")//Pop up message when email is sent with "Send Message"
                    .button("Back to Main Menu", go_back_to_main_dialog)); 
            }),
    );
   
}

fn sub_layer(siv: &mut Cursive, to: &str) {
    siv.pop_layer();

    //Copies the recipient string in a pointer
    let to_copy = Rc::new(to.to_owned());

    //Makes a clone of the copied string
    let to_copy_clone = Rc::clone(&to_copy);
    siv.add_layer(
        Dialog::new()
            .title("Subject")
            .content(
                EditView::new()
                    .on_submit(move |siv, subject| {
                        mess_layer(siv, &to_copy_clone, subject); //Goes to message layer when user presses enter
                    })
                    .with_name("subject")
                    .fixed_width(20),
            )
            .button("Next", move |siv| {
                let subject = siv
                    .call_on_name("subject", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                mess_layer(siv, &to_copy, &subject) //Goes to message layer when user presses "Next"
            }),
    );
}

fn send_layer(siv: &mut Cursive) {
    siv.pop_layer();
    
    //In order to send email it goes through the 3 pieces needed to send an email
    //the recipient:to, subject of the email:subject, message within email:message
    //Goes one by one, so input of to -> input of subject -> input of message
    siv.add_layer(
        Dialog::new()
            .title("Enter recipient's email")
            .content(
                EditView::new()
                    .on_submit(sub_layer) //Goes to subject layer when user presses enter
                    .with_name("to")
                    .fixed_width(20),
            )
            .button("Next", |siv| {
                let to = siv
                    .call_on_name("to", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                sub_layer(siv, &to); //Goes to subject layer when user presses "Next button"
            }),
    );


}

// , recipient: &str, subject: &str, message: &str
fn send(_: &mut Cursive, to: &str, subject: &str, message: &str) {
    let output = Command::new("python3")
        .arg("gmailCleanAPI.py")
        .arg(to)
        .arg(subject)
        .arg(message)
        .output()
        .expect("failed to execute process");
    // equivalent to running python3 gmailCleanAPI.py "to" "subject" "message"
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

fn calendar_error(siv: &mut Cursive) {

    siv.pop_layer();

    let error = Dialog::new()
    .content(TextView::new("Input Error"))
    .button("Back", calendar);

    siv.add_layer(error);


}

fn read_calendar_string(siv: &mut Cursive, arguments: [String; 12]) -> String {

    let mut command = Command::new("python3").args(arguments).spawn().expect("error");
    command.wait().expect("error");


    let mut file = std::fs::File::create("file.txt").expect("error");

    let mut display_string = "".to_string();

    // Reads the information in calendar.txt

    let file_text = fs::read_to_string("calendar.txt")
        .expect("calendar.txt not read");

    
    // Text that is left to be searched 
    let mut text_left = &file_text[0..file_text.len()];

    // text that is left 
    let mut bar_index_option = None;
    let mut bar_index = 1;

    let mut going = true; 
    while (going) {

        bar_index_option = text_left.find("|");

        // get day
        match bar_index_option {
            Some(x) => bar_index = x,
            None => calendar_error(siv),
        }

        let day = &text_left[0..bar_index];

        text_left = &text_left[bar_index+1..];

        bar_index_option = text_left.find("|");

        match bar_index_option {
            Some(x) => bar_index = x,
            None => calendar_error(siv),
        }

        let title = &text_left[0..bar_index];

        if (bar_index + 2 < text_left.len()) {

            text_left = &text_left[bar_index+2..];
        
        } else {

            going = false;

        }

        display_string = [display_string.to_string(), day.to_string(), title.to_string(), "\n".to_string()].join(" ");

        file.write_all(title.as_bytes()).expect("error");
        file.write_all(b"\n").expect("error");

    }

    return display_string;

}

fn calendar_weekly(siv: &mut Cursive) {

    siv.pop_layer();

    let arguments = ["calendar_week.py".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
    let display_string = read_calendar_string(siv, arguments);

    let events = Dialog::new()
    .content(TextView::new(display_string))
    .button("Back", calendar);
    
    siv.add_layer(events);

}


fn calendar_date(siv: &mut Cursive) {

    siv.pop_layer();

    let arguments = ["calendar_date.py".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
    let display_string = read_calendar_string(siv, arguments);

    let events = Dialog::new()
    .content(TextView::new(display_string))
    .button("Back", calendar);
    
    siv.add_layer(events);


}

fn separate_date(text: String) -> [String; 3] {

    if (text.len() == 10) {

        let year = (&text[0..4]).to_string();
        let month = (&text[5..7]).to_string();
        let day = (&text[8..10]).to_string();
        
        return [year, month, day];

    } else {

        return ["".to_string(), "".to_string(), "".to_string()];

    }


}

fn calendar_find(siv: &mut Cursive) {

    siv.pop_layer();

    siv.add_layer(

        Dialog::default().title("yyyy/mm/dd")
        .padding_lrtb(2, 2, 4, 4)
        
        .content(EditView::new().content("").with_name("yyyy/mm/dd").fixed_width(20))

        .button("Back", calendar)
    
        .button("Enter", |s| {


            let text = s.call_on_name("yyyy/mm/dd", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();

            let date = separate_date(text);

            if (date[0] != "".to_string()) {

                let year = (&date[0]).to_string();
                let month = (&date[1]).to_string();
                let day = (&date[2]).to_string();
                
                let arguments = ["calendar_find.py".to_string(), year, month, day];
                let display_string = read_calendar_find(s, arguments);
            
                let events = Dialog::new()
                .content(TextView::new(display_string))
                .button("Back", calendar);
                
                s.pop_layer();
                s.add_layer(events);

            } else {

                calendar_error(s);

            }

        })
    
    );

}

fn read_calendar_find(siv: &mut Cursive, arguments: [String; 4]) -> String {
    
    let mut command = Command::new("python3").args(arguments).spawn().expect("error");
    command.wait().expect("error");


    let mut file = std::fs::File::create("file.txt").expect("error");

    let mut display_string = "".to_string();

    // Reads the information in calendar.txt

    let file_text = fs::read_to_string("calendar.txt")
        .expect("calendar.txt not read");

    
    // Text that is left to be searched 
    let mut text_left = &file_text[0..file_text.len()];

    // text that is left 
    let mut bar_index_option = None;
    let mut bar_index = 1;

    let mut going = true; 
    while (going) {

        bar_index_option = text_left.find("|");

        // get day
        match bar_index_option {
            Some(x) => bar_index = x,
            None => calendar_error(siv),
        }

        let day = &text_left[0..bar_index];

        text_left = &text_left[bar_index+1..];

        bar_index_option = text_left.find("|");

        match bar_index_option {
            Some(x) => bar_index = x,
            None => calendar_error(siv),
        }

        let title = &text_left[0..bar_index];

        if (bar_index + 2 < text_left.len()) {

            text_left = &text_left[bar_index+2..];
        
        } else {

            going = false;

        }

        display_string = [display_string.to_string(), day.to_string(), title.to_string(), "\n".to_string()].join(" ");

        file.write_all(title.as_bytes()).expect("error");
        file.write_all(b"\n").expect("error");

    }

    return display_string;
    
}

fn calendar_name(siv: &mut Cursive) -> () {

    siv.pop_layer();

    let mut result = "".to_string();
    siv.add_layer(

        Dialog::new().title("Name")
        .content(EditView::new().content("").with_name("name").fixed_width(20))
        .button("Cancel", calendar)
        .button("Next", |s| {

            let mut text = s.call_on_name("name", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();

            start_date(s, text);

        })

    );


}

fn start_date(siv: &mut Cursive, name: String) -> () {

    siv.pop_layer();

    siv.add_layer(

        Dialog::new().title("Start yyyy/mm/dd")
        .content(EditView::new().content("").with_name("start").fixed_width(20))
        .button("Cancel", calendar)
        .button("Next", move |s| {

            let mut text = s.call_on_name("start", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();

            let date = separate_date(text);
            let next = [(&name).to_string(), (&date[0]).to_string(), (&date[1]).to_string(), (&date[2]).to_string()];

            if (date[0] != "") {

                calendar_end_date(s, next)

            } else {

                calendar_error(s);

            }

        })

    );

}

fn calendar_end_date(siv: &mut Cursive, prior:[String; 4]){
    siv.pop_layer();

    siv.add_layer(

        Dialog::new().title("End yyyy/mm/dd (Enter Q if Same Date)")
        .content(EditView::new().content("").with_name("start").fixed_width(20))
        .button("Cancel", calendar)
        .button("Next", move |s| {

            let mut text = s.call_on_name("start", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();
            
            if (text == "Q" || text == "q"){
                let next = [(&prior[0]).to_string(), (&prior[1]).to_string(), (&prior[2]).to_string(), (&prior[3]).to_string(), 
                    (&prior[1]).to_string(), (&prior[2]).to_string(), (&prior[3]).to_string()];
                calendar_start_time(s, next);
            }
            
            else if (text.len() == 10) {
                let date = separate_date(text);
                let next = [(&prior[0]).to_string(), (&prior[1]).to_string(), (&prior[2]).to_string(), (&prior[3]).to_string(), 
                    (&date[0]).to_string(), (&date[1]).to_string(), (&date[2]).to_string()];
                calendar_start_time(s, next);

            } else {

                calendar_error(s);

            }

        })

    );
    
}

fn calendar_start_time(siv: &mut Cursive, prior: [String; 7]) {

    siv.pop_layer();

    siv.add_layer(

        Dialog::new().title("Start hh:mm")
        .content(EditView::new().content("").with_name("start").fixed_width(20))
        .button("Cancel", calendar)
        .button("Next", move |s| {

            let mut text = s.call_on_name("start", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();

            if (text.len() == 5) {
                let time = separate_time(text);
                let next = [(&prior[0]).to_string(), (&prior[1]).to_string(), (&prior[2]).to_string(), (&prior[3]).to_string(), 
                (&prior[4]).to_string(), (&prior[5]).to_string(), (&prior[6]).to_string(), (&time[0]).to_string(), (&time[1]).to_string()];
                calendar_end_time(s, next)

            } else {

                calendar_error(s);

            }

        })

    );
}



fn calendar_end_time(siv: &mut Cursive, prior: [String; 9]) {

    siv.pop_layer();

    siv.add_layer(

        Dialog::new().title("End hh:mm")
        .content(EditView::new().content("").with_name("end").fixed_width(20))
        .button("Cancel", calendar)
        .button("Next", move |s| {

            let mut text = s.call_on_name("end", |view: &mut EditView| {

                view.get_content()

            }).unwrap().to_string();

            if (text.len() == 5) {
                let time = separate_time(text);
                let format = [(&prior[1]).to_string(), (&prior[2]).to_string(), 
                (&prior[3]).to_string(), (&prior[4]).to_string(), (&prior[5]).to_string(), (&prior[6]).to_string(), 
                (&prior[7]).to_string(), (&prior[8]).to_string(), (&time[0]).to_string(), (&time[1]).to_string(), 
                (&prior[0]).to_string(), "".to_string()];
                let mut command = Command::new("python3").arg("calendar_add_event.py").args(format).spawn().expect("error");
                calendar(s);

            } else {

                calendar_error(s);

            }

        })

    );

}

fn separate_time(text: String) -> [String; 2] {

    if (text.len() == 5) {

        let hour = (&text[0..2]).to_string();
        let minute = (&text[3..5]).to_string();
        
        return [hour, minute];

    } else {

        return ["".to_string(), "".to_string()];

    }


}

fn calendar_add(siv: &mut Cursive) {

    siv.pop_layer();

    calendar_name(siv);

}

fn calendar(siv: &mut Cursive) {
    siv.pop_layer();

    let options = Dialog::new()
    .content(TextView::new("Options"))
    .button("Weekly", calendar_weekly)
    .button("Daily", calendar_date)
    .button("Find", calendar_find)
    .button("Add", calendar_add)
    .button("Back", go_back_to_main_dialog);
        
    siv.add_layer(options);
    

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

