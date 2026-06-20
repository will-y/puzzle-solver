use leptos::prelude::*;
use star_puzzle::generator::generate_board;
use crate::boardcomponent::BoardComponent;

#[component]
pub fn PlayPage() -> impl IntoView {
    let board_signal = RwSignal::new(generate_board(2, 0.2).unwrap());

    view! {
        <div class="flex flex-col justify-center">
            <h1 class="text-center w-full">Manual Play</h1>
            <BoardComponent board=board_signal manual=true/>
        </div>
    }
}