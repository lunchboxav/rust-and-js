#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello, Mozilla!";
    js! {
        alert( @{message} );
    }

    stdweb::event_loop();
}