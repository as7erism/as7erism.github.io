use std::{cmp::Reverse, collections::BinaryHeap, rc::Rc};

use phf::phf_map;
use yew::html;

use crate::{
    fs::{FsIndex, FsTree}, HistoryHandle, StatusCode
};

pub type Program = fn(&[&str], &mut FsIndex, &mut FsTree, HistoryHandle) -> StatusCode;

pub const PROGRAMS: phf::Map<
    &'static str, Program
> = phf_map! {
    "cd" => cd,
    "help" => help,
};

fn cd(
    args: &[&str],
    cwd: &mut FsIndex,
    fs_tree: &mut FsTree,
    _history: HistoryHandle,
) -> StatusCode {
    StatusCode(0)
}

fn help(
    _args: &[&str],
    _cwd: &mut FsIndex,
    _fs_tree: &mut FsTree,
    history: HistoryHandle,
) -> StatusCode {
    let mut history_vec = history.to_vec();
    history_vec.last_mut().unwrap().output = html! {
        <>
            {
                for PROGRAMS
                    .keys()
                    .map(|k| Reverse(*k))
                    .collect::<BinaryHeap<_>>()
                    .into_iter_sorted()
                    .map(|r| r.0)
            }
        </>
    };

    history.set(history_vec);

    StatusCode(0)
}
