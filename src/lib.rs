#![feature(binary_heap_into_iter_sorted)]
use std::{cell::RefCell, error::Error, rc::Rc};

use unix_path::{Path, PathBuf};
use yew::{Html, UseStateHandle};

use crate::{
    fs::{FsError, FsIndex, FsTree},
    programs::{PROGRAMS, Program},
};

pub mod components;
pub mod fs;
pub mod programs;

pub const HOME: &str = "/home/user";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusCode(pub u32);

impl StatusCode {
    fn is_success(&self) -> bool {
        self.0 == 0
    }

    fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionRecord {
    last_status: StatusCode,
    cwd_display: Rc<str>,
    command: Rc<str>,
    pub output: Html,
}

impl ExecutionRecord {
    pub fn new(last_status: StatusCode, cwd_display: &str, command: &str, output: Html) -> Self {
        Self {
            last_status,
            cwd_display: cwd_display.into(),
            command: command.into(),
            output,
        }
    }

    pub fn last_status(&self) -> StatusCode {
        self.last_status
    }

    pub fn cwd_display(&self) -> Rc<str> {
        self.cwd_display.clone()
    }

    pub fn command(&self) -> Rc<str> {
        self.command.clone()
    }
}

pub type HistoryHandle = UseStateHandle<Vec<ExecutionRecord>>;

pub fn tab_complete(input: &str) -> String {
    // TODO
    format!("{input}, tab completed!")
}

pub fn get_program(
    name: &str,
    cwd: &PathBuf,
    fs_tree: &FsTree,
) -> Result<Option<&'static Program>, FsError> {
    // TODO
    Ok(PROGRAMS.get(name))
}

pub fn submit_command(
    command: &str,
    cwd: &mut PathBuf,
    fs_tree: Rc<RefCell<FsTree>>,
    history: &mut Vec<ExecutionRecord>,
) -> StatusCode {
    if let Ok(tokens) = shellish_parse::parse(command, false) {
        if let Some((name, args)) = tokens.split_at_checked(1) {
            let program = match get_program(name[0].as_str(), cwd, &fs_tree.borrow()) {
                Ok(Some(f)) => f,
                Ok(None) => unimplemented!(),
                Err(e) => unimplemented!(),
            };

            program(args, cwd, &mut fs_tree.borrow_mut(), history)
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }
}

// TODO this can probably return &str
pub fn display_path(path: &Path) -> Rc<str> {
    let path = path.to_string_lossy();
    if path == HOME {
        "~".into()
    } else {
        (*path).into()
    }
}

pub fn set_up_fs() -> FsTree {
    FsTree::default()
}
