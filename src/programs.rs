use std::{cmp::Reverse, collections::BinaryHeap};

use phf::phf_map;
use yew::{UseStateHandle, html};

use crate::{
    ExecutionRecord, StatusCode,
    fs::{FsNodeIndex, FsTree},
};

fn cd(
    args: &[&str],
    cwd: &mut FsNodeIndex,
    fs_tree: &mut FsTree,
    _history: UseStateHandle<Vec<ExecutionRecord>>,
) -> StatusCode {
    StatusCode(0)
}

fn help(
    _args: &[&str],
    _cwd: &mut FsNodeIndex,
    _fs_tree: &mut FsTree,
    history: UseStateHandle<Vec<ExecutionRecord>>,
) -> StatusCode {
    let mut history_vec = history.to_vec();
    history_vec.last_mut().unwrap().output = html! {
            <>
                { for PROGRAMS.keys().map(|k| Reverse(*k)).collect::<BinaryHeap<_>>().into_iter_sorted().map(|r| r.0) }
            </>
        };

    StatusCode(0)
}

pub const PROGRAMS: phf::Map<
    &'static str,
    fn(&[&str], &mut FsNodeIndex, &mut FsTree, UseStateHandle<Vec<ExecutionRecord>>) -> StatusCode,
> = phf_map! {
    "cd" => cd,
    "help" => help,
};
