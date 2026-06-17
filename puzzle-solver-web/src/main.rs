mod boardcomponent;

use leptos::prelude::*;
use star_puzzle::board::Board;
use star_puzzle::generator::generate_board;
use crate::boardcomponent::BoardComponent;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // let board_signal = RwSignal::new(Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap());

    let board_signal = RwSignal::new(generate_board(2).unwrap());

    view! {
        <BoardComponent board=board_signal/>
    }
}
