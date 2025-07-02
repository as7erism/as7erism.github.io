#![feature(binary_heap_into_iter_sorted)]
use std::rc::Rc;

use yew::{Html, UseStateHandle};

use crate::{fs::{FsError, FsIndex, FsTree}, programs::{Program, PROGRAMS}};

pub mod components;
pub mod fs;
pub mod programs;

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

pub fn tokenize(input: &str) -> Vec<&str> {
    // TODO support quotes?
    input.split_whitespace().collect()
}

pub fn tab_complete(input: &str) -> String {
    // TODO
    format!("{input}, tab completed!")
}

pub fn get_program(name: &str, cwd: &str, fs_tree: &FsTree) -> Result<Option<&'static Program>, FsError> {
    // TODO
    Ok(PROGRAMS.get("help"))
}
