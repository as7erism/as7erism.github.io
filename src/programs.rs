use std::{cmp::Reverse, collections::BinaryHeap, rc::Rc};

use phf::phf_map;
use unix_path::PathBuf;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{html, Html};

use crate::{
    ExecutionRecord, HistoryHandle, StatusCode,
    fs::{FsIndex, FsTree},
};

pub type Program =
    fn(&[String], &mut PathBuf, &mut FsTree, &mut Vec<ExecutionRecord>) -> StatusCode;

pub const PROGRAMS: phf::Map<&'static str, Program> = phf_map! {
    "cd" => cd,
    "clear" => clear,
    "echo" => echo,
    "help" => help,
    "ls" => ls,
};

fn ls(
    args: &[String],
    cwd: &mut PathBuf,
    fs_tree: &mut FsTree,
    history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    console::log_1(&JsValue::from_str(cwd.to_str().unwrap()));
    write_output(history, html! {
        <>
            {
                for fs_tree
                    .iter_dir(fs_tree.lookup_path(cwd).unwrap().unwrap())
                    .unwrap()
                    .map(|entry| Reverse(entry.name()))
                    .collect::<BinaryHeap<_>>()
                    .into_iter_sorted()
                    .map(|r| html! {<span>{format!("{} ", r.0)}</span>})
            }
        </>
    });

    StatusCode(0)
}

fn cd(
    args: &[String],
    cwd: &mut PathBuf,
    fs_tree: &mut FsTree,
    _history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    StatusCode(1)
}

fn help(
    _args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    history.last_mut().unwrap().output = html! {
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

    StatusCode(0)
}

fn echo(
    args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    history.last_mut().unwrap().output = html! {
        <>{ args.join(" ") }</>
    };
    StatusCode(0)
}

fn clear(
    _args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    history.clear();
    StatusCode(0)
}

fn write_output(history: &mut Vec<ExecutionRecord>, output: Html) {
    history.last_mut().unwrap().output = output;
}
