#![feature(binary_heap_into_iter_sorted)]
use std::rc::Rc;

use yew::Html;

pub mod components;
pub mod fs;
pub mod programs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusCode(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionRecord {
    pub command: Rc<str>,
    pub output: Html,
}
