use site::{
    components::{Fastfetch, Prompt}, display_path, fs::FsTree, init_fs, submit_command, tab_complete, ExecutionRecord, History, StatusCode, HOME
};
use unix_path::{Path, PathBuf};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(init_fs);

    let cwd_handle = use_state(|| PathBuf::from(HOME));
    let status_handle = use_state(|| StatusCode(0));
    let history_handle = use_state(|| {
        History(vec![ExecutionRecord::new(
            StatusCode(0),
            &display_path(&cwd_handle),
            "fastfetch",
            Some(html! {<><Fastfetch /></>}),
        )])
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
                let mut history = (*history_handle).clone();

                history.0.push(ExecutionRecord::new(
                    *status_handle,
                    &display_path(&cwd),
                    command.as_str(),
                    None,
                ));

                status_handle.set(submit_command(
                    &command,
                    &mut cwd,
                    fs_tree.clone(),
                    &mut history,
                ));

                // make sure cwd still exists, if not reset to root
                if fs_tree.borrow().lookup_path(&cwd).is_some() {
                    cwd_handle.set(cwd);
                } else {
                    cwd_handle.set(PathBuf::from("/"));
                };

                input_handle.set(String::new());
                history_handle.set(history);
            }
            _ => (),
        })
    };

    html! {
        <div>
            {
                for history_handle.0.iter().map(|record| {
                    html! {
                        <>
                            <Prompt status={record.last_status()} cwd_display={record.cwd_display()} />
                            <span>{record.command()}</span>
                            <br />
                            if record.output().is_some() {
                                // as far as i can tell an Rc would not work here
                                {record.output().unwrap().clone()}
                                <br />
                            }
                        </>
                    }
                })
            }
            <Prompt status={*status_handle} cwd_display={display_path(&(*cwd_handle).clone())} />
            <input id={"commandInput"} class={classes!("focus:outline-none")} onkeydown={handle_keydown} value={(*input_handle).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<Ash>::new().render();
}
