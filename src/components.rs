use yew::{classes, function_component, html, AttrValue, Html, Properties, props};

use crate::{fs::{FsIndex, FsTree}, StatusCode};

#[derive(Properties, PartialEq)]
pub struct PromptProps {
    pub status: StatusCode,
    pub cwd_display: AttrValue,
}

#[function_component]
pub fn Prompt(props: &PromptProps) -> Html {
    html! {
        <>
            <span class={classes!(if props.status.is_success() {"text-emerald-500"} else {"text-rose-500"})}>{"❁"}</span>
            {props.cwd_display.clone()}{" "}
        </>
    }
}
