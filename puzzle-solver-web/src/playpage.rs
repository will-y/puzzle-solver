use leptos::prelude::*;
use leptos_use::{use_interval, UseIntervalReturn};
use star_puzzle::generator::generate_board;
use crate::boardcomponent::BoardComponent;
use crate::navbar::NavBar;

#[component]
pub fn PlayPage() -> impl IntoView {
    let board_signal = RwSignal::new(generate_board(2, 0.2).unwrap());
    let UseIntervalReturn { counter, reset, .. } = use_interval(1000);
    let counter_formatted = move || format_as_time(counter.get());

    view! {
        <div>
            <NavBar center=view!{<p>{counter_formatted}</p>}.into_any() />
            <div class="p-4 flex flex-col justify-center">
                <h1 class="text-center w-full">Manual Play</h1>
                <BoardComponent board=board_signal manual=true reset_timer=reset.into() />
            </div>
        </div>
    }
}

fn format_as_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}