use web_sys::wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn eval(s: &str);
}

#[function_component]
fn Ash() -> Html {
    let history_handle = use_state(|| {
        vec![html! {
            <div />
        }]
    });

    let onclick = {
        let history = history_handle.clone();
        Callback::from(move |_: MouseEvent| {
            eval("console.log('hi');");
            let mut updated = history.to_vec();
            updated.push(html! {
                <p>{"waa"}</p>
            });
            history.set(updated);
        })
    };

    html! {
        <div>
            { for history_handle.to_vec() }
            {"❁~"}<input />
            <button onclick={onclick} class={classes!("bg-sky-500")}>
                {"yay"}
            </button>
        </div>
    }
}

fn main() {
    let hi = vec![String::from("awa")];
    yew::Renderer::<Ash>::new().render();
}
