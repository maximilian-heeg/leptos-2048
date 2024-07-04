use crate::game::{Actions, Game};
use leptos::*;

use leptos_use::use_raf_fn_with_options;
use leptos_use::utils::Pausable;
use leptos_use::UseRafFnOptions;

#[component]
pub fn RenderControls() -> impl IntoView {
    let setter = use_context::<WriteSignal<Game>>().expect("to have found the setter provided");
    let getter = use_context::<ReadSignal<Game>>().expect("to have found the getter provided");

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn_with_options(
        move |_| super::handle_monte_carlo(getter, setter),
        UseRafFnOptions::default().immediate(false),
    );

    let button_text = move || {
        if is_active() {
            "Pause"
        } else {
            "Start"
        }
    };

    view! {
        <div class="controls">
        <button
            on:click=move |_| super::handle_step(setter, Actions::Left)
            inner_html="&larr;"
        />

        <button
            on:click=move |_| super::handle_step(setter, Actions::Right)
            inner_html="&rarr;"
        />
        <button
            on:click=move |_| super::handle_step(setter, Actions::Up)
            inner_html="&uarr;"
        />
        <button
            on:click=move |_| super::handle_step(setter, Actions::Down)
            inner_html="&darr;"
       />
    </div>
    <div class="controls c-2">
       <button
           on:click=move |_| super::handle_monte_carlo(getter, setter)
      >
      MCTS <br/>  1 move (Space)
      </button>
      <button on:click=move |_|
          {if is_active() {pause()} else {resume()}}>
          {button_text}
          </button>
    </div>
    }
}
