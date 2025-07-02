use std::{ops::DerefMut, rc::Rc};

use site::{
    components::Prompt, fs::{FsIndex, FsTree}, get_program, tab_complete, tokenize, ExecutionRecord, StatusCode
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(FsTree::new);

    let cwd_handle = use_state(String::default);
    let status_handle = use_state(|| StatusCode(0));
    let history_handle = use_state(|| vec![ExecutionRecord::new(StatusCode(0), "~", "neofetch", html!{<>{"hi"}</>})]);
    let input_handle = use_state(String::default);

    let handle_keydown = {
        let cwd_handle = cwd_handle.clone();
        let status_handle = status_handle.clone();
        let history_handle = history_handle.clone();
        let input_handle = input_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Tab" => {
                    e.prevent_default();
                    input_handle.set(tab_complete(e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().as_str()));
                },
                "Enter" => {
                    e.prevent_default();
                    let command = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
                    let tokens = tokenize(command.as_str());

                    let mut history = history_handle.to_vec();
                    history.push(ExecutionRecord::new(*status_handle, cwd_handle.as_str(), command.as_str(), html! {<></>}));
                    history_handle.set(history);
                    input_handle.set("".to_string());

                    let mut cwd = (*cwd_handle).clone();
                    if let Some((name, args)) = tokens.split_at_checked(1) {
                        let program = match get_program(name[0], cwd_handle.as_str(), &fs_tree.borrow()) {
                            Ok(Some(f)) => f,
                            Ok(None) => unimplemented!(),
                            Err(e) => unimplemented!(),
                        };
                        program(args, &mut cwd, fs_tree.borrow_mut().deref_mut(), history_handle.clone());
                    } else {
                        status_handle.set(StatusCode(0));
                    };
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
                <Prompt status={*status_handle} cwd_display={(*cwd_handle).clone()} />
                <input onkeydown={handle_keydown} value={(*input_handle).clone()} />
            </div>
        }
}

fn main() {
    yew::Renderer::<Ash>::new().render();
}
