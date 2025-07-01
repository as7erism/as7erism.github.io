use std::rc::Rc;

use site::{
    components::Prompt, fs::{FsIndex, FsTree}, tab_complete, ExecutionRecord, StatusCode
};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(FsTree::new);
    let cwd = use_mut_ref(|| fs_tree.borrow().root());

    let history_handle = use_state(|| vec![ExecutionRecord::new(StatusCode(0), "~", "neofetch", html!{<>{"hi"}</>})]);
    let input_handle = use_state(String::default);

    let handle_keydown = {
        let input_handle = input_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Tab" => {
                    e.prevent_default();
                    let target = e.target().expect("event should have a target..");
                    input_handle.set(tab_complete(target.unchecked_into::<HtmlInputElement>().value().as_str()));
                },
                "Enter" => {
                    e.prevent_default();
                    input_handle.set("awa".into());
                },
                _ => (),
            }
        })
    };

    html! {
            <div>
                {
                    for history_handle.iter().map(|record| {
                        html! {
                            <>
                                <Prompt status={record.last_status()} cwd_display={record.cwd_display()} />
                                <span>{record.command()}</span>
                                <br />
                                {record.output.clone()}
                                <br />
                            </>
                        }
                    })
                }
                <Prompt status={StatusCode(0)} cwd_display={Rc::from("~")} />
                <input onkeydown={handle_keydown} value={(*input_handle).clone()} />
            </div>
        }
}

fn main() {
    yew::Renderer::<Ash>::new().render();
}
