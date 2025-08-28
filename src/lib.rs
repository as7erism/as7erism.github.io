#![feature(binary_heap_into_iter_sorted)]
use std::{cell::RefCell, error::Error, ffi::OsStr, rc::Rc};

use programs::EXECUTE_FILE;
use unix_path::{Path, PathBuf};
use unix_str::UnixStr;
use web_sys::console;
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
    output: Option<Html>,
}

impl ExecutionRecord {
    pub fn new(
        last_status: StatusCode,
        cwd_display: &str,
        command: &str,
        output: Option<Html>,
    ) -> Self {
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

    pub fn output(&self) -> Option<&Html> {
        self.output.as_ref()
    }

    // TODO write function here?
}

#[derive(Clone, Debug, PartialEq)]
pub struct History(pub Vec<ExecutionRecord>);

impl History {
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn write(&mut self, output: Html) -> Result<(), ()> {
        self.0.last_mut().ok_or(())?.output = output.into();
        Ok(())
    }
}

pub type HistoryHandle = UseStateHandle<History>;

pub fn tab_complete(input: &str) -> String {
    // TODO
    format!("{input}, tab completed!")
}

pub fn get_program(
    name: &str,
    cwd: &PathBuf,
    fs_tree: &FsTree,
) -> Result<Option<&'static Program>, FsError> {
    if name.contains('/') {
        Ok(Some(&EXECUTE_FILE))
    } else {
        Ok(PROGRAMS.get(name))
    }
}

pub fn submit_command(
    command: &str,
    cwd: &mut PathBuf,
    fs_tree: Rc<RefCell<FsTree>>,
    history: &mut History,
) -> StatusCode {
    if let Ok(tokens) = shellish_parse::parse(command, false) {
        let args = if tokens.is_empty() {
            Vec::from([String::new()])
        } else {
            tokens
        };

        let program = match get_program(args[0].as_str(), cwd, &fs_tree.borrow()) {
            Ok(Some(f)) => f,
            Ok(None) => unimplemented!(),
            Err(e) => unimplemented!(),
        };

        program(&args, cwd, &mut fs_tree.borrow_mut(), history)
    } else {
        unimplemented!();
    }
}

pub fn display_path(path: &Path) -> Rc<str> {
    // TODO justify this
    let path = path.to_string_lossy();
    (*if path.starts_with(HOME) {
        path.replacen(HOME, "~", 1).into()
    } else {
        path
    })
    .into()
}

pub fn canonicalize(path: &PathBuf, fs_tree: &FsTree) -> Result<PathBuf, ()> {
    if path.is_relative() {
        unimplemented!()
    }

    let mut out = PathBuf::from("/");

    let mut current = fs_tree.root();
    for component in path
        .iter()
        .skip(1)
        .filter(|c| !c.is_empty() && *c != UnixStr::new("."))
    {
        if component == UnixStr::new("..") {
            out.pop();
        } else {
            out.push(component);
        }
        match fs_tree.iter_dir(current) {
            Ok(mut iter) => {
                current = iter
                    .find(|entry| entry.name().as_ref() == component.to_str().unwrap())
                    .ok_or(())?
                    .index();
            }
            Err(_) => unimplemented!(),
        }
    }

    Ok(out)
}

pub fn init_fs() -> FsTree {
    FsTree::default()
}
