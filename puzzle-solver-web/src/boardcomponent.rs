use std::collections::HashSet;
use std::f64::consts::PI;
use star_puzzle_solver::solver::Solver;
use leptos::html::Canvas;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::js_sys::Math::{cos, sin};
use web_sys::MouseEvent;
use star_puzzle::board::Board;
use star_puzzle_solver::solver::rulesolver::{RuleSolver};
use web_sys::console;

const COLORS: [&str; 15] = [
    "#FF5733", // Vibrant Red-Orange
    "#33FF57", // Bright Lime Green
    "#3357FF", // Electric Blue
    "#F39C12", // Warm Orange
    "#9B59B6", // Amethyst Purple
    "#1ABC9C", // Turquoise Cyan
    "#E74C3C", // Coral Red
    "#2ECC71", // Emerald Green
    "#3498DB", // Sky Blue
    "#F1C40F", // Sunflower Yellow
    "#D35400", // Pumpkin Orange
    "#8E44AD", // Deep Purple
    "#16A085", // Teal Blue-Green
    "#C0392B", // Strong Dark Red
    "#27AE60", // Jade Green
];

const SQUARE_SIZE: usize = 30;

#[component]
pub fn BoardComponent(
    board: RwSignal<Board>,
    #[prop(default = false)]
    manual: bool) -> impl IntoView {
    let board_size = move || board.read().size;
    let canvas_ref = NodeRef::<Canvas>::new();

    // TODO: This is not great, just use a <pre> with what I was doing before?
    let (solver_results, set_solver_result) = signal(vec![]);
    let (errors, set_errors) = signal(HashSet::<(usize, usize)>::new());

    Effect::new(move |_| {
        if let Some(node) = canvas_ref.get() {
            let context = node
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            draw_board(&board.read(), &context, &errors);
        }
    });

    // TODO: Size the board better
    view! {
        <div class="flex flex-col justify-center align-center gap-8">
            <div class="flex justify-center align-center">
                <canvas class="grow-0" width=move || {board_size() * SQUARE_SIZE} height=move || {board_size() * SQUARE_SIZE} node_ref=canvas_ref on:click=move |event| on_board_clicked(event, board)></canvas>
            </div>
            <Show when=move || manual>
                <div class="flex justify-center align-center gap-4">
                    <button class="btn btn-warning" on:click=move |_| clear(board, set_errors)>Clear</button>
                    <button class="btn btn-success" on:click=move |_| check(board, set_errors)>Check</button>
                </div>
            </Show>
            <Show when=move || !manual>
                <button on:click=move |_| solve_board(board, set_solver_result)>
                    "Solve"
                </button>
            </Show>
            <p>
                <ul>
                    <For
                        each=move || solver_results.get().into_iter()
                        key=move |result| result.id.clone()
                        children=move |result| {
                            view! {
                                <li>{result.value}</li>
                            }
                        }
                    />
                 </ul>
            </p>
            // <pre>{move || board.read().to_string()}</pre>
        </div>
    }
}

fn clear(board: RwSignal<Board>, set_errors: WriteSignal<HashSet<(usize, usize)>>) {
    board.update(|board| {
        board.clear();
    });

    set_errors.set(HashSet::new());
}

fn check(board: RwSignal<Board>, set_errors: WriteSignal<HashSet<(usize, usize)>>) {
    let errors = board.read().check_board().expect("Can't check board");
    set_errors.set(errors);
}

fn solve_board(board: RwSignal<Board>, set_solver_result: WriteSignal<Vec<StringListEntry>>) {
    board.update(|board| {
        let solver = RuleSolver::default();
        let solver_result = solver.solve(board);
        set_solver_result.set(solver_result.format_results().split('\n').map(|s| StringListEntry::new(s.to_string())).collect());
    });
}

fn on_board_clicked(event: MouseEvent, board: RwSignal<Board>) {
    let board_x = (event.offset_x() as f32 / SQUARE_SIZE as f32).floor() as usize;
    let board_y = (event.offset_y() as f32 / SQUARE_SIZE as f32).floor() as usize;

    if event.shift_key() {
        board.update(|board| {
            board.place_star(board_x, board_y).expect("Failed to place star");
        });
    } else {
        board.update(|board| {
            board.place_dot(board_x, board_y);
        });
    }

    if board.read().is_solved() {
        console::log_1(&"Board is solved!".into());
    }
}

fn draw_board(board: &Board, context: &web_sys::CanvasRenderingContext2d, errors: &ReadSignal<HashSet<(usize, usize)>>) {
    let color_map = board.create_color_map();
    let errors = errors.read();

    context.set_stroke_style_str("#000000");
    for x in 0..board.size {
        for y in 0..board.size {
            context.set_fill_style_str(COLORS[color_map[y][x]]);
            context.fill_rect(x as f64 * SQUARE_SIZE as f64, y as f64 * SQUARE_SIZE as f64, SQUARE_SIZE as f64, SQUARE_SIZE as f64);
            context.begin_path();
            context.rect(x as f64 * SQUARE_SIZE as f64, y as f64 * SQUARE_SIZE as f64, SQUARE_SIZE as f64, SQUARE_SIZE as f64);
            context.stroke();

            if board.has_star(x, y) {
                draw_star(context, x * SQUARE_SIZE + SQUARE_SIZE / 2, y * SQUARE_SIZE + SQUARE_SIZE / 2, !errors.is_empty() && errors.contains(&(x, y)));
            } else if board.has_dot(x, y) {
                draw_dot(context, x * SQUARE_SIZE + SQUARE_SIZE / 2, y * SQUARE_SIZE + SQUARE_SIZE / 2);
            }
        }
    }
}

fn draw_star(context: &web_sys::CanvasRenderingContext2d, x: usize, y: usize, error: bool) {
    let mut rot = PI / 2.0 * 3.0;
    let step = PI / 5.0;
    let outer_radius = SQUARE_SIZE as f64 / 3.0;
    let inner_radius = outer_radius / 2.5;

    let mut current_x;
    let mut current_y;

    context.begin_path();
    context.move_to(x as f64, y as f64 - outer_radius);

    for _ in 0..5 {
        current_x = x as f64 + cos(rot) * outer_radius;
        current_y = y as f64 + sin(rot) * outer_radius;
        context.line_to(current_x, current_y);
        rot += step;

        current_x = x as f64 + cos(rot) * inner_radius;
        current_y = y as f64 + sin(rot) * inner_radius;
        context.line_to(current_x, current_y);
        rot += step;
    }

    context.line_to(x as f64, y as f64 - outer_radius);
    context.close_path();
    context.stroke();
    context.set_fill_style_str(if error { "#FF0000" } else { "#000000" });
    context.fill();
}

fn draw_dot(context: &web_sys::CanvasRenderingContext2d, x: usize, y: usize) {
    context.begin_path();
    context.arc(x as f64, y as f64, 3.0, 0.0, 2.0 * PI).unwrap();
    context.set_fill_style_str("#000000");
    context.fill();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct StringListEntry {
    value: String,
    id: String
}

impl StringListEntry {
    fn new(value: String) -> Self {
        Self { value, id: uuid::Uuid::new_v4().to_string() }
    }
}