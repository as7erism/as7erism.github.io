use std::{cmp::Reverse, collections::BinaryHeap, rc::Rc};

use phf::phf_map;
use web_sys::console;
use yew::html;

use crate::{
    fs::{FsIndex, FsTree}, HistoryHandle, StatusCode
};

pub type Program = fn(&[&str], &mut String, &mut FsTree, HistoryHandle) -> StatusCode;

pub const PROGRAMS: phf::Map<
    &'static str, Program
> = phf_map! {
    "cd" => cd,
    "help" => help,
};

fn cd(
    args: &[&str],
    cwd: &mut String,
    fs_tree: &mut FsTree,
    _history: HistoryHandle,
) -> StatusCode {
    StatusCode(0)
}

fn help(
    _args: &[&str],
    _cwd: &mut String,
    _fs_tree: &mut FsTree,
    history: HistoryHandle,
) -> StatusCode {
    let mut history_vec = history.to_vec();
    console::log_1(&format!("{}", history_vec.len()).into());
    history_vec.last_mut().unwrap().output = html! {
        <>
            {
                for PROGRAMS
                    .keys()
                    .map(|k| Reverse(*k))
                    .collect::<BinaryHeap<_>>()
                    .into_iter_sorted()
                    .map(|r| html! {<span>{format!("{} ", r.0)}</span>})
            }
        </>
    };

    history.set(history_vec);

    StatusCode(0)
}
