use yew::{AttrValue, Html, Properties, classes, function_component, html, props};

use crate::{
    StatusCode,
    fs::{FsIndex, FsTree},
};

#[derive(Properties, PartialEq)]
pub struct PromptProps {
    pub status: StatusCode,
    pub cwd_display: AttrValue,
}

#[function_component]
pub fn Prompt(props: &PromptProps) -> Html {
    html! {
        <>
            {props.cwd_display.clone()}
            {" "}
            <span class={classes!(if props.status.is_success() {"text-green-300"} else {"text-rose-400"})}>{"❁"}</span>
            {" "}
        </>
    }
}

#[function_component]
pub fn Fastfetch() -> Html {
    html! {
        <div class={classes!("flex", "wrap-anywhere", "max-w-xl")}>
            <img src={"static/jirachi.png"} alt={"jirachi!"} class={classes!("self-center", "size-40")} />
            <div>
                <span class={classes!("text-purple-400", "font-bold")}>{"user"}</span>{"@"}<span class={classes!("text-purple-400", "font-bold")}>{"asters-pc"}</span>
                <br />
                {"--------------"}
                <br />
                {"hi, im aster! i'm a fourth-year university student studying computer science. my particular interests include rust, programming language design and web development."}
                <br /><br />
                {"welcome to my site! you can navigate it like you would a unix terminal. enter `help` below to get started..."}
                <br /><br />
                <span class="font-bold">{"github: "}</span><a href={"https://github.com/as7erism"} class={classes!("underline", "font-bold", "text-blue-400", "hover:text-blue-300", "hover:text-shadow-blue-400", "hover:text-shadow-xs")}>{"as7erism"}</a>
                <br />
                <span class="font-bold">{"email: "}</span><a href={"mailto:astermayhew@proton.me"} class={classes!("underline", "font-bold", "text-blue-400", "hover:text-blue-300", "hover:text-shadow-blue-400", "hover:text-shadow-xs")}>{"astermayhew@proton.me"}</a>
                <br /><br />
                <pre class={"leading-none"}>
                    <span class="bg-gray-900">{"   "}</span>
                    <span class="bg-rose-400">{"   "}</span>
                    <span class="bg-green-300">{"   "}</span>
                    <span class="bg-amber-200">{"   "}</span>
                    <span class="bg-blue-300">{"   "}</span>
                    <span class="bg-violet-400">{"   "}</span>
                    <span class="bg-sky-400">{"   "}</span>
                    <span class="bg-fuchsia-100">{"   "}</span>
                </pre>
                <pre class={"leading-none"}>
                    <span class="bg-gray-800">{"   "}</span>
                    <span class="bg-red-400">{"   "}</span>
                    <span class="bg-emerald-300">{"   "}</span>
                    <span class="bg-yellow-200">{"   "}</span>
                    <span class="bg-blue-400">{"   "}</span>
                    <span class="bg-purple-400">{"   "}</span>
                    <span class="bg-cyan-400">{"   "}</span>
                    <span class="bg-fuchsia-50">{"   "}</span>
                </pre>
            </div>
        </div>
    }
}
