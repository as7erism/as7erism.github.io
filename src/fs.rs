use std::{collections::HashMap, marker::PhantomData, mem, rc::Rc};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use yew::Html;
use web_sys::{js_sys::eval, wasm_bindgen::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Directory {
    entries: HashMap<Rc<str>, FsNodeIndex>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct DirEntry {
    name: Rc<str>,
    index: FsNodeIndex,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct File {
    contents: String,
}

impl Directory {
    pub fn new_root() -> Self {
        Default::default()
    }

    pub fn new(parent: FsNodeIndex, this: FsNodeIndex) -> Self {
        Self {
            entries: HashMap::from([("..".into(), parent), (".".into(), this)]),
        }
    }

    fn children(&self) -> impl Iterator<Item = DirEntry> {
        self.entries.iter().map(|(k, v)| DirEntry {
            name: Rc::clone(k),
            index: *v,
        })
    }

    pub fn execute(&self) -> (Html, u32) {
        unimplemented!();
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            entries: HashMap::from([("..".into(), FsNodeIndex(0)), (".".into(), FsNodeIndex(0))]),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct JsProgramResult {
    output: String,
    #[serde(rename = "returnCode")]
    return_code: u32,
}

impl File {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn contents_mut(&mut self) -> &mut String {
        &mut self.contents
    }

    pub fn contents(&self) -> &str {
        self.contents.as_str()
    }

    pub fn execute(&self) -> (Html, u32) {
        // match eval(self.contents()) {
        //     Ok(return_value) => {
        //         let a = serde_wasm_bindgen::from_value::<JsProgramResult>(return_value);
        //     }
        // }
        unimplemented!();
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum FsNode {
    Directory(Directory),
    File(File),
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct FsNodeIndex(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, Serialize, Deserialize)]
pub enum FsError {}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct FsTree {
    node_table: Vec<FsNode>,
    vacancies: Vec<usize>,
}

impl FsTree {
    pub fn new() -> Self {
        FsTree {
            node_table: vec![FsNode::Directory(Directory::default())],
            vacancies: Vec::new(),
        }
    }

    fn get_node(&self, index: FsNodeIndex) -> &FsNode {
        &self.node_table[index.0]
    }

    fn get_node_mut(&mut self, index: FsNodeIndex) -> &mut FsNode {
        &mut self.node_table[index.0]
    }

    pub fn root(&self) -> FsNodeIndex {
        FsNodeIndex(0)
    }

    fn vacate(&mut self, index: FsNodeIndex) {
        self.vacancies.push(index.0)
    }

    fn is_child(&self, maybe_child: FsNodeIndex, maybe_parent: FsNodeIndex) -> bool {
        unimplemented!()
    }

    pub fn move_entry(
        &mut self,
        old_name: &str,
        old_parent: FsNodeIndex,
        new_name: &str,
        new_parent: FsNodeIndex,
    ) -> Result<(), FsError> {
        unimplemented!();
    }

    pub fn create_directory(&mut self, name: &str, parent: FsNodeIndex) -> Result<FsNodeIndex, FsError> {
        let vacancy = self.vacancies.last().cloned();
        let table_len = self.node_table.len();

        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            unimplemented!();
        };

        if parent_dir.entries.contains_key(name) {
            unimplemented!();
        }

        match vacancy {
            Some(v) => {
                let node_index = FsNodeIndex(v);
                parent_dir.entries.insert(name.into(), node_index);
                self.node_table[v] = FsNode::Directory(Directory::new(parent, node_index));
                self.vacancies.pop();
                Ok(node_index)
            }
            None => {
                let node_index = FsNodeIndex(table_len);
                parent_dir
                    .entries
                    .insert(name.into(), node_index);
                self.node_table.push(FsNode::Directory(Directory::new(
                    parent,
                    node_index,
                )));
                Ok(node_index)
            }
        }
    }

    pub fn create_file(&mut self, name: &str, parent: FsNodeIndex) -> Result<FsNodeIndex, FsError> {
        let vacancy = self.vacancies.last().cloned();
        let table_len = self.node_table.len();

        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            unimplemented!();
        };

        if parent_dir.entries.contains_key(name) {
            unimplemented!();
        }

        match vacancy {
            Some(v) => {
                let node_index = FsNodeIndex(v);
                parent_dir.entries.insert(name.into(), node_index);
                self.node_table[v] = FsNode::File(File::new());
                self.vacancies.pop();
                Ok(node_index)
            }
            None => {
                let node_index = FsNodeIndex(table_len);
                parent_dir
                    .entries
                    .insert(name.into(), node_index);
                self.node_table.push(FsNode::File(File::new()));
                Ok(node_index)
            }
        }
    }

    pub fn delete(&mut self, name: &str, parent: FsNodeIndex) -> Result<(), FsError> {
        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            unimplemented!();
        };

        let Some(removal_index) = parent_dir.entries.remove(name) else {
            unimplemented!();
        };

        self.vacate(removal_index);

        Ok(())
    }

    pub fn delete_recursive(&mut self, name: &str, parent: FsNodeIndex) -> Result<(), FsError> {
        let FsNode::Directory(parent_dir) = self.get_node(parent) else {
            unimplemented!();
        };

        let Some(removal_index) = parent_dir.entries.get(name).cloned() else {
            unimplemented!();
        };

        if let FsNode::Directory(d) = self.get_node(removal_index) {
            let names = d.entries.keys().map(Rc::clone).collect::<Vec<_>>();
            for name in names {
                self.delete_recursive(&name, removal_index).expect("a");
            }
        }

        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            panic!();
        };
        parent_dir.entries.remove(name).expect("a");
        self.vacate(removal_index);

        Ok(())
    }
}
