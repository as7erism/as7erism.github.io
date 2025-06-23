use core::panic;
use std::{collections::HashMap, marker::PhantomData, mem, rc::Rc};

use thiserror::Error;

struct Directory {
    entries: HashMap<Rc<str>, FsNodeIndex>,
}

struct DirEntry {
    name: Rc<str>,
    index: FsNodeIndex,
}

#[derive(Default)]
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

    fn remove_entry(&mut self, name: &str) -> Option<FsNodeIndex> {
        self.entries.remove(name)
    }

    fn children(&self) -> impl Iterator<Item = DirEntry> {
        self.entries.iter().map(|(k, v)| DirEntry {name: Rc::clone(k), index: *v})
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            entries: HashMap::from([("..".into(), FsNodeIndex(0)), (".".into(), FsNodeIndex(0))]),
        }
    }
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
}

enum FsNode {
    Directory(Directory),
    File(File),
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
struct FsNodeIndex(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error)]
pub enum FsError {}

struct FsTree {
    node_table: Vec<FsNode>,
    vacancies: Vec<FsNodeIndex>,
}

impl FsTree {
    pub fn new() -> Self {
        FsTree {
            node_table: vec![FsNode::Directory(Directory::default())],
            vacancies: Vec::new(),
        }
    }

    fn get_node(&self, entry: FsNodeIndex) -> &FsNode {
        &self.node_table[entry.0]
    }

    fn get_node_mut(&mut self, entry: FsNodeIndex) -> &mut FsNode {
        &mut self.node_table[entry.0]
    }

    pub fn root(&self) -> FsNodeIndex {
        FsNodeIndex(0)
    }

    fn vacate(&mut self, entry: FsNodeIndex) {
        self.vacancies.push(entry)
    }

    fn is_child(&self, maybe_child: FsNodeIndex, maybe_parent: FsNodeIndex) -> bool {
        unimplemented!()
    }

    pub fn move_entry(
        &mut self,
        entry: FsNodeIndex,
        new_parent: FsNodeIndex,
        new_name: &str,
    ) -> Result<(), ()> {
        if self.is_child(new_parent, entry) {
            return Err(());
        }
        Ok(())
    }

    pub fn create_file(&mut self, name: &str, parent: FsNodeIndex) -> Result<(), FsError> {
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
                parent_dir.entries.insert(name.into(), v);
                self.node_table[v.0] = FsNode::File(File::new());
                self.vacancies.pop();
            },
            None => {
                parent_dir.entries.insert(name.into(), FsNodeIndex(table_len));
                self.node_table.push(FsNode::File(File::new()));
            },
        };

        Ok(())
    }

    pub fn delete(&mut self, name: &str, parent: FsNodeIndex) -> Result<(), FsError> {
        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            unimplemented!();
        };

        let Some(removal_index) = parent_dir.remove_entry(name) else {
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
        parent_dir.remove_entry(name).expect("a");
        self.vacate(removal_index);

        Ok(())
    }
}
