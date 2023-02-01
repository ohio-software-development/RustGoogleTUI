use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, DummyView, EditView, ListView};
use cursive::With;
use cursive::{Cursive, CursiveExt};

#[derive(Clone, Debug, Default)]
struct User {
    usern: String,
    passw: String,
}

fn main() {
    let mut siv = Cursive::default();
    siv.set_user_data(User::default());
    siv.add_global_callback('Q', |s| s.quit());

    siv.add_layer(
        Dialog::text("Do you want to login or quit?")
            .button("Login", login)
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}

/*
 * Need to implement a way to provide credentials
 * for now lets just store
 * Maybe like this?
 *
 */
fn login(s: &mut Cursive) {
    let current_data = s
        .with_user_data(|user_data: &mut User| user_data.clone())
        .unwrap();
    s.add_layer(
        Dialog::text("Please enter your username and password for Blackboard.")
            .title("Blackboard Login")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                    .child("", DummyView)
                    .child(
                        "Username: ",
                        EditView::new()
                            .content(current_data.usern.clone())
                            .with_name("Username")
                            .fixed_width(20),
                    )
                    .child("", DummyView)
                    .child(
                        "Password:",
                        EditView::new()
                            .content(current_data.passw.clone())
                            .with_name("Password")
                            .fixed_width(20),
                    ),
            )
            .button("Cancel", |s| {
                s.pop_layer();
            }), // .button("Login", sendinfo()),
    );
    // this is a placeholder until we figure out the blackboard api
}

// fn sendinfo() {}
