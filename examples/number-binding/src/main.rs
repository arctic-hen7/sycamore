use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, 10.0);

    view! { cx,
        p {
            (format!("{:.2}",value.get()))
        }

        input(type="range", min="1", step="0.25", max="10", bind:valueAsNumber=value) {}
        br {}
        input(type="number", min="1", step="0.25", max="10", bind:valueAsNumber=value) {}
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
