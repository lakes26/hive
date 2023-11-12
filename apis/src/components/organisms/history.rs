use crate::providers::game_state::GameStateSignal;
use hive_lib::{color::Color, game_result::GameResult, game_status::GameStatus};
use leptos::*;
use leptos_icons::{
    AiIcon::{
        AiFastBackwardFilled, AiFastForwardFilled, AiStepBackwardFilled, AiStepForwardFilled,
    },
    Icon,
};

#[component]
pub fn HistoryMove(turn: usize, piece: String, position: String) -> impl IntoView {
    let mut game_state_signal = expect_context::<GameStateSignal>();

    let onclick = move |_| {
        game_state_signal.show_history_turn(turn);
    };
    let get_class = move || {
        let mut class = "ml-3 hover:bg-blue-300 col-span-2";
        if let Some(history_turn) = game_state_signal.signal.get().history_turn {
            if turn == history_turn {
                class = "ml-3 hover:bg-blue-300 col-span-2 bg-orange-300"
            }
        }
        class
    };
    view! {
        <div class=get_class on:click=onclick>
            {format!("{}. {piece} {position}", turn + 1)}
        </div>
    }
}

#[component]
pub fn History(#[prop(optional)] extend_tw_classes: &'static str) -> impl IntoView {
    let mut game_state_signal = expect_context::<GameStateSignal>();

    let history_moves = move || {
        let mut his = Vec::new();
        for (i, (piece, pos)) in game_state_signal
            .signal
            .get()
            .state
            .history
            .moves
            .into_iter()
            .enumerate()
        {
            if i == 0 {
                his.push((i, piece, String::new()));
            } else {
                his.push((i, piece, pos));
            }
        }
        his
    };

    let is_finished = move || {
        matches!(
            game_state_signal.signal.get().state.game_status,
            GameStatus::Finished(_)
        )
    };

    let game_result = move || match (game_state_signal.signal)().state.game_status {
        GameStatus::Finished(result) => match result {
            GameResult::Draw => "½-½",
            GameResult::Winner(Color::White) => "1-0",
            GameResult::Winner(Color::Black) => "0-1",
            _ => "",
        },
        _ => "",
    };

    let next = move |_| {
        game_state_signal.next_history_turn();
    };

    let previous = move |_| {
        game_state_signal.previous_history_turn();
    };

    let first = move |_| {
        game_state_signal.first_history_turn();
    };

    let last = move |_| {
        game_state_signal.view_history();
    };

    let button_styles =
        "flex justify-center box-content inline-block text-center cursor-pointer hover:bg-green-300 mt-6 rounded-md border-cyan-500 border-2 drop-shadow-lg";
    let white_black_styles = "ml-3 mt-6 mb-3 col-span-2";
    let icon_style = "";
    view! {
        <div class=format!("grid grid-cols-4 gap-1 {extend_tw_classes}")>
            <div class="col-span-4 grid grid-cols-4 gap-1 sticky top-0 dark:bg-gray-900 bg-white">
                <button class=button_styles on:click=first>
                    <Icon icon=Icon::from(AiFastBackwardFilled) class=icon_style/>
                </button>

                <button class=button_styles on:click=previous>
                    <Icon icon=Icon::from(AiStepBackwardFilled) class=icon_style/>
                </button>

                <button class=button_styles on:click=next>
                    <Icon icon=Icon::from(AiStepForwardFilled) class=icon_style/>
                </button>

                <button class=button_styles on:click=last>
                    <Icon icon=Icon::from(AiFastForwardFilled) class=icon_style/>
                </button>

                <div class=white_black_styles>White</div>

                <div class=white_black_styles>Black</div>
            </div>
            <For each=history_moves key=|history_move| (history_move.0) let:history_move>

                <HistoryMove turn=history_move.0 piece=history_move.1 position=history_move.2/>
            </For>

            <Show when=is_finished>
                <div class="col-span-4 text-center">{game_result().to_string()}</div>
            </Show>
        </div>
    }
}

