use crate::common::{TournamentAction, UserAction};
use crate::components::molecules::{
    game_previews::GamePreviews, invite_user::InviteUser, time_row::TimeRow, user_row::UserRow,
};
use crate::providers::{
    navigation_controller::NavigationControllerSignal, tournaments::TournamentStateSignal,
    ApiRequests, AuthContext,
};
use chrono::Local;
use leptos::*;
use leptos_router::use_navigate;
use shared_types::{GameSpeed, TimeInfo, TournamentStatus};

const BUTTON_STYLE: &str = "flex gap-1 justify-center items-center px-4 py-2 font-bold text-white rounded bg-button-dawn dark:bg-button-twilight hover:bg-pillbug-teal active:scale-95 disabled:opacity-25 disabled:cursor-not-allowed disabled:hover:bg-transparent";

#[component]
pub fn Tournament() -> impl IntoView {
    let navi = expect_context::<NavigationControllerSignal>();
    let tournaments = expect_context::<TournamentStateSignal>();
    let tournament_id = move || navi.tournament_signal.get().tournament_id;
    let current_tournament = move || {
        tournament_id().and_then(|tournament_id| {
            tournaments
                .signal
                .get()
                .tournaments
                .get(&tournament_id)
                .cloned()
        })
    };
    let auth_context = expect_context::<AuthContext>();
    let account = move || match (auth_context.user)() {
        Some(Ok(Some(account))) => Some(account),
        _ => None,
    };
    let number_of_players = move || current_tournament().map_or(0, |t| t.players.len());
    let user_joined = move || {
        if let Some(account) = account() {
            current_tournament()
                .map_or(false, |t| t.players.iter().any(|(id, _)| *id == account.id))
        } else {
            false
        }
    };
    let user_is_organizer = move || {
        if let Some(account) = account() {
            current_tournament().map_or(false, |t| t.organizers.iter().any(|p| p.uid == account.id))
        } else {
            false
        }
    };

    let delete = move |_| {
        if let Some(tournament_id) = tournament_id() {
            if user_is_organizer() {
                let action = TournamentAction::Delete(tournament_id);
                let api = ApiRequests::new();
                api.tournament(action);
                let navigate = use_navigate();
                navigate("/tournaments", Default::default());
            }
        }
    };
    let start = move |_| {
        if let Some(tournament_id) = tournament_id() {
            if user_is_organizer() {
                let action = TournamentAction::Start(tournament_id);
                let api = ApiRequests::new();
                api.tournament(action);
            }
        }
    };
    let leave = move |_| {
        if let Some(tournament_id) = tournament_id() {
            let api = ApiRequests::new();
            api.tournament(TournamentAction::Leave(tournament_id));
        }
    };

    let join = move |_| {
        if let Some(tournament_id) = tournament_id() {
            let api = ApiRequests::new();
            api.tournament(TournamentAction::Join(tournament_id));
        }
    };

    let display_tournament = move || {
        current_tournament().and_then(|tournament| {
            let time_info = TimeInfo{mode:tournament.time_mode.clone() ,base: tournament.time_base, increment: tournament.time_increment};
            let tournament = store_value(tournament);
            let start_disabled = move || {let tournament =tournament(); tournament.min_seats > tournament.players.len() as i32} ;
            let join_disabled = move || {
                let tournament= tournament();
                if tournament.invite_only || tournament.seats <= tournament.players.len() as i32 {
                    return true;
                }
                if let Some(account) = account() {
                    let user = account.user;
                    let game_speed =
                    GameSpeed::from_base_increment(tournament.time_base, tournament.time_increment);
                    let rating = user.rating_for_speed(&game_speed) as i32;
                    match (tournament.band_lower, tournament.band_upper) {
                        (None, None) => false,
                        (None, Some(upper)) => rating >= upper,
                        (Some(lower), None) => rating <= lower,
                        (Some(lower), Some(upper)) => rating <= lower || rating >= upper,
                    }
                } else {true}

            };
            let user_kick = move || {
                if user_is_organizer() {
                    vec![UserAction::Kick(Box::new(tournament()))]
                } else {
                    vec![]
                }
            };
            let user_uninvite = move || {
                if user_is_organizer() {
                    vec![UserAction::Uninvite(tournament().tournament_id)]
                } else {
                    vec![]
                }
            };
            let starts = move || {
                let tournament = tournament();
                if matches!(tournament.status, TournamentStatus::NotStarted) {
                    match tournament.start_at {
                        None => "Start up to organizer".to_string(),
                        Some(time) => time
                            .with_timezone(&Local)
                            .format("Starts: %d/%m/%Y %H:%M")
                            .to_string(),
                    }
                } else {
                let pretty = tournament.status.pretty_string();
                if let Some(started_at) = tournament.started_at {
                        let start = started_at.with_timezone(&Local)
                        .format("started: %d/%m/%Y %H:%M")
                        .to_string();
                        format! ("{pretty}, {start}")
                    } else {pretty}
                }
            };
            let not_started = move || tournament().status == TournamentStatus::NotStarted;
            view! {
                <h1 class="place-self-center p-2 text-3xl font-bold">{tournament().name}</h1>
                <div class="overflow-y-auto w-60 md:w-[720px] max-h-96 flex justify-center">
                    {tournament().description}
                </div>
                <div>
                    <p class="font-bold">Tournament details:</p>
                    <div class="flex gap-1">"Time control: " <TimeRow time_info/></div>
                    <div>"Players: " {number_of_players} / {tournament().seats}</div>
                    <Show when=not_started>
                        <div>"Minimum players: " {tournament().min_seats}</div>
                    </Show>
                    <div>{starts}</div>
                </div>
                <Show when=not_started>
                    <div class="flex gap-1 justify-center items-center pb-2">
                        <Show
                            when=user_joined
                            fallback=move || {
                                view! {
                                    <button
                                        prop:disabled=join_disabled
                                        class=BUTTON_STYLE
                                        on:click=join
                                    >
                                        Join
                                    </button>
                                }
                            }
                        >

                            <button class=BUTTON_STYLE on:click=leave>
                                Leave
                            </button>
                        </Show>
                        <Show when=user_is_organizer>
                            <button class=BUTTON_STYLE on:click=delete>
                                {"Delete"}
                            </button>
                            <button prop:disabled=start_disabled class=BUTTON_STYLE on:click=start>
                                {"Start"}
                            </button>
                        </Show>
                    </div>
                </Show>
                <div class="flex flex-col flex-wrap place-content-center md:flex-row">
                    <div class="flex flex-col">
                        <div class="flex flex-col items-center">
                            <p class="font-bold">Organizers</p>
                            <For
                                each=move || { tournament().organizers }

                                key=|users| (users.uid)
                                let:user
                            >
                                <div>
                                    <UserRow actions=vec![] user=store_value(user)/>
                                </div>
                            </For>
                        </div>

                    </div>
                    <Show
                        when=move || tournament().status != TournamentStatus::NotStarted
                        fallback=move || {
                            view! {
                                <div class="flex flex-col items-center px-1 w-72">
                                    <Show when=move || !tournament().players.is_empty()>
                                        <p class="font-bold">Players</p>
                                        <For
                                            each=move || { tournament().players }

                                            key=|(id, _)| (*id)
                                            let:user
                                        >
                                            <UserRow actions=user_kick() user=store_value(user.1)/>
                                        </For>
                                    </Show>
                                </div>
                                <div class="flex flex-col items-center px-1 w-72">
                                    <Show when=move || !tournament().invitees.is_empty()>
                                        <p class="font-bold">Invitees</p>
                                        <For
                                            each=move || { tournament().invitees }
                                            key=|users| (users.uid)
                                            let:user
                                        >
                                            <UserRow actions=user_uninvite() user=store_value(user)/>
                                        </For>
                                    </Show>
                                    <Show when=user_is_organizer>
                                        <p class="font-bold">Invite players</p>
                                        <InviteUser tournament=tournament()/>
                                    </Show>
                                </div>
                            }
                        }
                    >

                        <div class="flex flex-col items-center w-full">
                            <p class="font-bold">Standings</p>
                            <For
                                each=move || { tournament().standings.into_iter() }

                                key=|(id, _)| (*id)
                                let:score
                            >

                                {
                                    let user = store_value(
                                        tournament()
                                            .players
                                            .get(&score.0)
                                            .expect("User in tournament")
                                            .clone(),
                                    );
                                    view! {
                                        <UserRow actions=vec![] user end_str=score.1.to_string()/>
                                    }
                                }

                            </For>
                            Tournament Games:
                            <div class="flex flex-wrap justify-center items-center">
                                <GamePreviews games=Callback::new(move |_| (tournament().games))/>
                            </div>
                        </div>
                    </Show>
                </div>
            }
            .into()
        })
    };
    view! {
        <div class="flex flex-col justify-center items-center pt-20 w-full">
            <div class="container flex flex-col items-center">{display_tournament}</div>
        </div>
    }
}
