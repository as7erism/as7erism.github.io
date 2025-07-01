use std::rc::Rc;

use site::{
    components::Prompt, fs::{FsIndex, FsTree}, ExecutionRecord, StatusCode
};
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(FsTree::new);
    let cwd = use_mut_ref(|| fs_tree.borrow().root());
    let history_handle = use_state(|| vec![ExecutionRecord::new(StatusCode(0), "~", "neofetch", html!{<>{"hi"}</>})]);
    let input_ref = use_node_ref();
    let input_handle = use_state(String::default);

    let handle_keydown = {
        let input_handle = input_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Tab" => {
                    e.prevent_default();
                    input_handle.set("hi".into());
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
                <input ref={input_ref} onkeydown={handle_keydown} value={(*input_handle).clone()} />
            </div>
        }
}

fn main() {
    yew::Renderer::<Ash>::new().render();
}
