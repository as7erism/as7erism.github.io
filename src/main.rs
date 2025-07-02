use site::{
    components::Prompt, display_path, fs::FsTree, submit_command, tab_complete, ExecutionRecord, StatusCode, HOME
};
use unix_path::{Path, PathBuf};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(FsTree::new);

    let cwd_handle = use_state(|| PathBuf::from(HOME));
    let status_handle = use_state(|| StatusCode(0));
    let history_handle = use_state(|| {
        vec![ExecutionRecord::new(
            StatusCode(0),
            &display_path(&cwd_handle),
            "neofetch",
            html! {<>{"hi"}</>},
        )]
    });
    let input_handle = use_state(String::default);

    let handle_keydown = {
        let cwd_handle = cwd_handle.clone();
        let status_handle = status_handle.clone();
        let history_handle = history_handle.clone();
        let input_handle = input_handle.clone();

        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "Tab" => {
                e.prevent_default();
                input_handle.set(tab_complete(
                    e.target()
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap()
                        .value()
                        .as_str(),
                ));
            }
            "Enter" => {
                e.prevent_default();
                let command = e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let mut cwd = (*cwd_handle).clone();
                let mut history = history_handle.to_vec();

                history.push(ExecutionRecord::new(
                    *status_handle,
                    &display_path(&cwd),
                    command.as_str(),
                    html! {<></>},
                ));

                status_handle.set(submit_command(
                    &command,
                    &mut cwd,
                    fs_tree.clone(),
                    &mut history,
                ));
                input_handle.set(String::default());
                cwd_handle.set(cwd);
                history_handle.set(history);
            }
            _ => (),
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
            <Prompt status={*status_handle} cwd_display={display_path(&(*cwd_handle).clone())} />
            <input onkeydown={handle_keydown} value={(*input_handle).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<Ash>::new().render();
}
