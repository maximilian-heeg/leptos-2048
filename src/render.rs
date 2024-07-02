use crate::alg;
use crate::game::*;
use leptos::logging::log;
use leptos::*;
use leptos_hotkeys::use_hotkeys;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};
use leptos_use::use_raf_fn_with_options;
use leptos_use::utils::Pausable;
use leptos_use::UseRafFnOptions;

fn handle_step(setter: WriteSignal<Game>, action: Actions) {
    setter.update(|game| {
        game.step(action);
    });
}

fn handle_monte_carlo(getter: ReadSignal<Game>, setter: WriteSignal<Game>) {
    let next_move = alg::monte_carlo(&getter.get());
    if let Some(action) = next_move {
        handle_step(setter, action)
    };
}

#[component]
pub fn RenderGame() -> impl IntoView {
    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    let (game, set_game) = create_signal(Game::new());

    use_hotkeys!(("ArrowUp") => move |_| handle_step(set_game, Actions::Up));
    use_hotkeys!(("ArrowDown") => move |_| handle_step(set_game, Actions::Down));
    use_hotkeys!(("ArrowLeft") =>  move |_| handle_step(set_game, Actions::Left));
    use_hotkeys!(("ArrowRight") =>  move |_| handle_step(set_game, Actions::Right));
    use_hotkeys!(("Space") => move |_| handle_monte_carlo(game, set_game));

    use_hotkeys!(("Keys") => move |_| {
        while !game().is_game_over(){
            let next_move = alg::monte_carlo(&game.get());
            if let Some(next_move) = next_move {
                set_game.update(|game| {game.step(next_move);});
            }
            log!("{}", game().highest_tile().unwrap());
        }

    });

    provide_context(set_game);
    provide_context(game);
    view! {
        <main _ref=main_ref>
            <h1> 2048</h1>
            <div class="score">Score: {move || game().score}</div>
            <RenderBoard game=game/>
            <RenderControls />
        </main>
    }
}

#[component]
fn RenderBoard(game: ReadSignal<Game>) -> impl IntoView {
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

#[component]
fn RenderControls() -> impl IntoView {
    let setter = use_context::<WriteSignal<Game>>().expect("to have found the setter provided");
    let getter = use_context::<ReadSignal<Game>>().expect("to have found the setter provided");

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn_with_options(
        move |_| handle_monte_carlo(getter, setter),
        UseRafFnOptions::default().immediate(false),
    );

    view! {
        <div class="controls">
        <button
            on:click=move |_| handle_step(setter, Actions::Left)
            inner_html="&larr;"
        />

        <button
            on:click=move |_| handle_step(setter, Actions::Right)
            inner_html="&rarr;"
        />
        <button
            on:click=move |_| handle_step(setter, Actions::Up)
            inner_html="&uarr;"
        />
        <button
            on:click=move |_| handle_step(setter, Actions::Down)
            inner_html="&darr;"
       />
    </div>
    <div class="controls c-1">
       <button
           on:click=move |_| handle_monte_carlo(getter, setter)
      >
      Monte Carlo Tree Search <br/> (Space)
      </button>
    </div>
    <div class="controls c-2">
        <button on:click=move |_| resume()>start</button>
        <button on:click=move |_| pause()>pause</button>
    </div>
    }
}
