use crate::game::Game;
use leptos::*;

#[component]
pub fn RenderBoard(game: ReadSignal<Game>) -> impl IntoView {
    let tiles = move || game().tiles();
    view! {
        <div class="game">
            <div class="board">
                <BoardBackground />
            </div>
            <div class="tiles">
                <For
                    each=move || tiles().into_iter()
                    key=|key| key.0
                    children=move |(index, _)| {
                            let value = create_memo(move |_| {
                                *tiles().get(&index).unwrap()
                            });

                            view! {
                                <div
                                id={index}
                                class={move || class_name(value().0, value().1, value().2)}
                                class:new={move || value().3}
                                class:changed={move || value().4}
                                >
                                    {move || value().2}
                                </div>
                            }
                        }
                />
            </div>
        </div>
    }
}

fn class_name(i: usize, j: usize, value: u32) -> String {
    format!("col_{j} row_{i} v_{value}")
}

#[component]
fn BoardBackground() -> impl IntoView {
    view! {
        <div>
            <div> </div>
            <div> </div>
            <div> </div>
            <div> </div>
        </div>
        <div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
        <div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
        <div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
    }
}
