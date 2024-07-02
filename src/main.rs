use leptos::*;
use leptos_2048::*;
use render::RenderGame;

fn main() {
    mount_to_body(|| {
        view! {
            <RenderGame />
        }
    })
}
