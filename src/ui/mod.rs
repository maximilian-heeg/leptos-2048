use crate::game::*;
use crate::mcts;
use leptos::*;
use leptos_hotkeys::use_hotkeys;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};

mod board;
use board::RenderBoard;

mod controls;

fn handle_step(setter: WriteSignal<Game>, action: Actions) {
    setter.update(|game| {
        game.step(action);
    });
}

fn handle_monte_carlo(getter: ReadSignal<Game>, setter: WriteSignal<Game>) {
    let next_move = mcts::simlulation(&getter.get());
    if let Some(action) = next_move {
        handle_step(setter, action)
    };
}

#[component]
pub fn RenderGame() -> impl IntoView {
    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    let (game, set_game) = create_signal(Game::new());
    provide_context(set_game);
    provide_context(game);

    use_hotkeys!(("ArrowUp") => move |_| handle_step(set_game, Actions::Up));
    use_hotkeys!(("ArrowDown") => move |_| handle_step(set_game, Actions::Down));
    use_hotkeys!(("ArrowLeft") =>  move |_| handle_step(set_game, Actions::Left));
    use_hotkeys!(("ArrowRight") =>  move |_| handle_step(set_game, Actions::Right));
    use_hotkeys!(("Space") => move |_| handle_monte_carlo(game, set_game));

    view! {
        <main _ref=main_ref>
            <h1> 2048</h1>
            <div class="score">Score: {move || game().score}</div>
            <RenderBoard game=game/>
            <controls::RenderControls />
        </main>
    }
}
