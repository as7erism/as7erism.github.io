use core::panic;
use std::{collections::HashMap, mem, rc::Rc};

struct Directory {
    entries: HashMap<Box<str>, FsNodeIndex>,
}

#[derive(Default)]
struct File {
    contents: String,
}

impl Directory {
    pub fn new_root() -> Self {
        Default::default()
    }

    fn remove_entry(&mut self, name: &str) -> Option<FsNodeIndex> {
        self.entries.remove(name)
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            entries: HashMap::from([
                ("..".into(), FsNodeIndex(0)),
                (".".into(), FsNodeIndex(0))
            ]),
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

struct CannotDeleteDirectory;
struct CannotDeleteRoot;

struct FsTree {
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
        self.vacancies.push(entry.0)
    }

    fn is_child(&self, maybe_child: FsNodeIndex, maybe_parent: FsNodeIndex) -> bool {
        unimplemented!()
    }

    pub fn move_entry(&mut self, entry: FsNodeIndex, new_parent: FsNodeIndex, new_name: &str) -> Result<(), ()> {
        if self.is_child(new_parent, entry) {
            return Err(());
        }
        Ok(())
    }

    pub fn delete_recursive(&mut self, name: &str, parent: FsNodeIndex) -> Result<(), CannotDeleteRoot> {
        let FsNode::Directory(parent_dir) = self.get_node_mut(parent) else {
            panic!();
        };

        let node = self.get_node(entry);
        match node {
            FsNode::File(f) => {
                match self.get_node_mut(f.parent()) {
                    FsNode::Directory(d) => d.remove_entry(entry),
                    FsNode::File(_) => panic!("parent was a file!"),
                };
            },
            FsNode::Directory(_) => {
                let mut entries = vec![];
                self.collect_entries(entry, &mut entries); 
                for entry in entries.into_iter().rev() {
                    if let FsNode::Directory(d) = self.get_node_mut(entry) {
                        d.entries.clear();
                    }
                    self.vacate(entry);
                }


            },
        };

        Ok(())
    }

    fn collect_entries<'a>(&self, current: FsNodeIndex, entries: &'a mut Vec<FsNodeIndex>) {
        entries.push(current);
        if let FsNode::Directory(d) = self.get_node(current) {
            for entry in d.entries.values() {
                self.collect_entries(*entry, entries);
            }
        }
    }
}
