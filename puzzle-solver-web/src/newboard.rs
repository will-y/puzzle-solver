use leptos::prelude::*;
use star_puzzle::board::Board;
use star_puzzle::generator::generate_board;

#[component]
pub fn NewBoard(board: RwSignal<Board>, reset_timer: Callback<()>) -> impl IntoView {
    let (difficulty, set_difficulty) = signal(30);
    let (star_count, set_star_count) = signal(2usize);

    view! {
        <button onclick="new_board_modal.showModal()" class="btn btn-square">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
        </button>
        <dialog id="new_board_modal" class="modal modal-bottom sm:modal-middle">
          <div class="modal-box">
            <h3 class="text-lg font-bold">Create New Board</h3>
            <h4 class="mt-2 mb-1">Difficulty</h4>
            <div class="w-full max-w-xs">
              <input on:input:target=move |ev| {set_difficulty.set(ev.target().value().parse().expect("Expected valid number"))}
                prop:value=difficulty type="range" min="5" max="100" value="30" class="range range-primary" step="5" />
              <div class="flex justify-between px-2.5 mt-2 text-xs">
                <span>|</span>
                <span>|</span>
                <span>|</span>
                <span>|</span>
                <span>|</span>
              </div>
              <div class="flex justify-between px-2.5 mt-2 text-xs">
                <span>1</span>
                <span>2</span>
                <span>3</span>
                <span>4</span>
                <span>5</span>
              </div>
            </div>
            <h4 class="mt-2 mb-1">Number of Stars</h4>
            <select on:input:target=move |ev| {set_star_count.set(ev.target().value().parse().expect("Expected valid number"))}
                prop:value=move || star_count.get().to_string()
                class="select select-primary">
              <option disabled>1</option>
              <option selected>2</option>
              <option disabled>3</option>
            </select>
            <div class="modal-action">
              <form method="dialog">
                <button on:click=move |_| {
                    on_new_board_clicked(board, difficulty, star_count);
                    reset_timer.run(());
                } class="btn btn-success">Create</button>
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">x</button>
              </form>
            </div>
          </div>
        </dialog>
    }
}

fn on_new_board_clicked(board_signal: RwSignal<Board>, difficulty: ReadSignal<i32>, star_count: ReadSignal<usize>) {
    let difficulty = difficulty.get() as f32 / 100.0;
    board_signal.set(generate_board(star_count.get(), difficulty).unwrap());
}
