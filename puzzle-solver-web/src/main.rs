mod boardcomponent;
mod playpage;
mod solvepage;
mod statspage;
mod navbar;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;
use star_puzzle::board::Board;
use crate::playpage::PlayPage;
use crate::solvepage::SolvePage;
use crate::statspage::StatsPage;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let board_signal = RwSignal::new(Board::from_string("aaccchhhhg\naaachhhggg\naaacdhgggg\naaacddiggg\naaccdiieee\nfcccdiieee\nffccdiieje\nfbbbbeeejj\nffbbeeejjj\nffbbbeejjj", 2).unwrap());
    // let board_signal = RwSignal::new(generate_board(2, 0.0).unwrap());
//<BoardComponent board=board_signal/>
    view! {
        <Router>
            <Main />
        </Router>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
                <input id="my-drawer-4" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content">
                    <div>
                        <Routes fallback=|| "Not found.">
                            <Route path=path!("/") view=PlayPage/>
                            <Route path=path!("/solve") view=SolvePage/>
                            <Route path=path!("/stats") view=StatsPage/>
                        </Routes>
                    </div>
                </div>
                <div class="drawer-side is-drawer-close:overflow-visible">
                    <label for="my-drawer-4" aria-label="close sidebar" class="drawer-overlay"></label>
                    <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
                        <ul class="menu w-full grow">
                            <li>
                              <a class:menu-active=move || use_location().pathname.get() == "/" class="is-drawer-close:tooltip is-drawer-close:tooltip-right" data-tip="Play" href="/">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
                                </svg>
                                <span class="is-drawer-close:hidden">Play</span>
                              </a>
                            </li>
                            <li>
                              <a class:menu-active=move || use_location().pathname.get() == "/solve" class="is-drawer-close:tooltip is-drawer-close:tooltip-right btn-secondary" data-tip="Run Solver" href="/solve">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z" />
                                </svg>
                                <span class="is-drawer-close:hidden">Run Solver</span>
                              </a>
                            </li>
                            <li>
                              <a class:menu-active=move || use_location().pathname.get() == "/stats" class="is-drawer-close:tooltip is-drawer-close:tooltip-right" data-tip="Solver Stats" href="/stats">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605" />
                                </svg>
                                <span class="is-drawer-close:hidden">Solver Stats</span>
                              </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
    }
}
