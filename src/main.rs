use std::rc::Rc;

use site::{
    ExecutionRecord,
    fs::{FsNodeIndex, FsTree},
};
use yew::prelude::*;

#[function_component]
fn Ash() -> Html {
    let fs_tree = use_mut_ref(FsTree::new);
    let cwd = use_mut_ref(|| fs_tree.borrow().root());
    let history_handle = use_state(Vec::<ExecutionRecord>::new);

    //let onclick = {
    //    let history = history_handle.clone();
    //    Callback::from(move |_: MouseEvent| {
    //        let mut updated = history.to_vec();
    //        updated.push(html! {
    //            <p>{"waa"}</p>
    //        });
    //        history.set(updated);
    //    })
    //};

    html! {
            <div>
    //            { for history_handle.to_vec() }
     //           {"❁~"}<input />
    //         <button onclick={onclick} class={classes!("bg-sky-500")}>
                    //{"yay"}
                //</button>
            </div>
        }
}

fn main() {
    let hi = vec![String::from("awa")];
    yew::Renderer::<Ash>::new().render();
}
