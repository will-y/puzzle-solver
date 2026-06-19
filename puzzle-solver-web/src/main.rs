mod boardcomponent;

use crate::boardcomponent::BoardComponent;
use leptos::prelude::*;
use star_puzzle::board::Board;
use star_puzzle::generator::generate_board;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let board_signal = RwSignal::new(Board::from_string("aaccchhhhg\naaachhhggg\naaacdhgggg\naaacddiggg\naaccdiieee\nfcccdiieee\nffccdiieje\nfbbbbeeejj\nffbbeeejjj\nffbbbeejjj", 2).unwrap());
    // let board_signal = RwSignal::new(generate_board(2, 0.0).unwrap());

    view! {
        <BoardComponent board=board_signal/>
    }
}
