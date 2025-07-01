use yew::{Html, function_component, html};

use crate::fs::{FsNodeIndex, FsTree};

#[function_component]
pub fn Prompt() -> Html {
    html!(<></>)
}

#[function_component]
pub fn InputLine() -> Html {
    html!(<></>)
}

fn get_full_path(tree: &FsTree, index: FsNodeIndex) {}
