use leptos::*;

use crate::{components::molecules::user_row::UserRow, providers::users::UserSignal};

#[component]
pub fn OnlineUsers() -> impl IntoView {
    let online_users = expect_context::<UserSignal>();
    let online_players = move || (online_users.signal)().username_user;
    let is_empty = move || online_players().is_empty();
    view! {
        <div class="flex flex-col m-2 w-fit pt-2 md:pt-6">
            Online players: <div>
                <div class=move || {
                    format!("p-1 h-6 {}", if !is_empty() { "hidden" } else { "flex" })
                }>{move || if is_empty() { "Only you" } else { "" }}</div>

                <For each=online_players key=|(key, _)| key.to_owned() let:a_user>
                    <UserRow user=store_value(a_user.1)/>
                </For>
            </div>
        </div>
    }
}
