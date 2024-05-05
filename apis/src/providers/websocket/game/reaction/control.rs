use super::handler::reset_game_state;
use crate::{
    common::server_result::GameActionResponse,
    providers::{
        alerts::AlertType, alerts::AlertsContext, game_state::GameStateSignal, games::GamesSignal,
        navigation_controller::NavigationControllerSignal, timer::TimerSignal,
    },
};
use hive_lib::{game_control::GameControl, game_result::GameResult, game_status::GameStatus};
use leptos::*;

pub fn handle_control(game_control: GameControl, gar: GameActionResponse) {
    let mut games = expect_context::<GamesSignal>();
    let navigation_controller = expect_context::<NavigationControllerSignal>();
    let mut game_state = expect_context::<GameStateSignal>();
    if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
        if gar.game.nanoid == nanoid {
            game_state.set_pending_gc(game_control.clone())
        }
    }
    //log!("Got a GC: {}", game_control);
    match game_control {
        GameControl::Abort(_) => {
            games.own_games_remove(&gar.game.nanoid);
            if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
                if gar.game.nanoid == nanoid {
                    let alerts = expect_context::<AlertsContext>();
                    alerts.last_alert.update(|v| {
                        *v = Some(AlertType::Warn(format!(
                            "{} aborted the game",
                            gar.username
                        )));
                    });
                    // TODO: Once we have notifications tell the user the game was aborted
                    let navigate = leptos_router::use_navigate();
                    navigate("/", Default::default());
                }
            }
        }
        GameControl::DrawAccept(_) => {
            games.own_games_remove(&gar.game.nanoid);
            if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
                if gar.game.nanoid == nanoid {
                    game_state.set_game_status(GameStatus::Finished(GameResult::Draw));
                    game_state.set_game_response(gar.game.clone());
                    let timer = expect_context::<TimerSignal>();
                    timer.update_from(&gar.game);
                }
            }
        }
        GameControl::Resign(color) => {
            games.own_games_remove(&gar.game.nanoid);
            if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
                if gar.game.nanoid == nanoid {
                    game_state.set_game_status(GameStatus::Finished(GameResult::Winner(
                        color.opposite_color(),
                    )));
                    game_state.set_game_response(gar.game.clone());
                    let timer = expect_context::<TimerSignal>();
                    timer.update_from(&gar.game);
                }
            }
        }
        GameControl::TakebackAccept(_) => {
            games.own_games_add(gar.game.to_owned());
            if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
                if gar.game.nanoid == nanoid {
                    let timer = expect_context::<TimerSignal>();
                    timer.update_from(&gar.game);
                    reset_game_state(&gar.game);
                }
            }
        }
        _ => {
            games.own_games_add(gar.game.to_owned());
            if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
                if gar.game.nanoid == nanoid {
                    let timer = expect_context::<TimerSignal>();
                    timer.update_from(&gar.game);
                    reset_game_state(&gar.game);
                }
            }
        }
    }
}
