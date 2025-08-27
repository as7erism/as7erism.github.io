use std::{cmp::Reverse, collections::BinaryHeap, rc::Rc};

use phf::phf_map;
use unix_path::{Path, PathBuf};
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{Html, html};

use crate::{
    canonicalize, fs::{FsIndex, FsTree}, ExecutionRecord, History, HistoryHandle, StatusCode
};

pub type Program = fn(&[String], &mut PathBuf, &mut FsTree, &mut History) -> StatusCode;

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
    history: &mut History,
) -> StatusCode {
    if history
        .write(html! {
            <>
                {
                    for fs_tree
                        .iter_dir(fs_tree.lookup_path(cwd).unwrap())
                        .unwrap()
                        .map(|entry| Reverse(entry.name()))
                        .collect::<BinaryHeap<_>>()
                        .into_iter_sorted()
                        .map(|r| html! {<span>{format!("{} ", r.0)}</span>})
                }
            </>
        })
        .is_ok()
    {
        StatusCode(0)
    } else {
        // TODO statuses
        StatusCode(1)
    }
}

fn cd(
    args: &[String],
    cwd: &mut PathBuf,
    fs_tree: &mut FsTree,
    _history: &mut History,
) -> StatusCode {
    if args.len() < 2 {
        unimplemented!()
    }

    let target_path = if args[1].starts_with('/') {
        PathBuf::from(&args[1])
    } else {
        let mut target_path = cwd.clone();
        target_path.push(&args[1]);
        target_path
    };

    match fs_tree.lookup_path(&target_path) {
        Some(index) => {
            if fs_tree.is_directory(index).unwrap() {
                *cwd = canonicalize(&target_path, fs_tree).unwrap();
                StatusCode(0)
            } else {
                unimplemented!()
            }
        },
        None => unimplemented!(),
    }
}

fn help(
    _args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut History,
) -> StatusCode {
    if history
        .write(html! {
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
        })
        .is_ok()
    {
        StatusCode(0)
    } else {
        // TODO statuses
        StatusCode(1)
    }
}

fn echo(
    args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut History,
) -> StatusCode {
    if history
        .write(html! {
            <>{ args[1..].join(" ") }</>
        })
        .is_ok()
    {
        StatusCode(0)
    } else {
        StatusCode(1)
    }
}

fn clear(
    _args: &[String],
    _cwd: &mut PathBuf,
    _fs_tree: &mut FsTree,
    history: &mut History,
) -> StatusCode {
    history.clear();
    StatusCode(0)
}

pub fn execute_file(
    args: &[String],
    cwd: &mut PathBuf,
    fs_tree: &mut FsTree,
    history: &mut History,
) -> StatusCode {
    let fs_result = if args[0].starts_with('/') {
        fs_tree.lookup_path(&Path::new(&args[0]))
    } else {
        let mut target_path = cwd.clone();
        target_path.push(&Path::new(&args[0]));
        fs_tree.lookup_path(&target_path)
    };
}
