use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use web_sys::{EventTarget, HtmlInputElement, wasm_bindgen::JsCast};
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let history_handle = use_state(|| {
        vec![html! {
            <div />
        }]
    });

    let onclick = {
        let history = (*history_handle).clone();
        Callback::from(move |_| {
            let mut updated = history.clone();
            updated.push(html! {
                <p>{"waa"}</p>
            });
            history_handle.set(updated);
        })
    };

    html! {
        <div>
            { for history_handle.to_vec() }
            {"❁~"}<input />
        </div>
    }
}

fn tab_complete<'a>(value: &str, program_names: impl IntoIterator<Item = &'a str>) {
    unimplemented!()
}

fn main() {
    let hi = vec![String::from("awa")];
    yew::Renderer::<Ash>::new().render();
}
