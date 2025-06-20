use core::panic;
use std::rc::Rc;

#[derive(Default)]
struct Directory {
    name: Rc<str>,
    parent: Option<DirEntry>,
    entries: Vec<DirEntry>,
}

#[derive(Default)]
struct File {
    name: Rc<str>,
    parent: DirEntry,
}

impl Directory {
    pub fn new() -> Self {
        Default::default()
    }

    fn remove_entry(&mut self, entry: DirEntry) -> Option<()> {
        let (index, _) = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, e)| **e == entry)?;
        self.entries.swap_remove(index);
        Some(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> Option<DirEntry> {
        self.parent
    }
}

impl File {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> DirEntry {
        self.parent
    }
}

enum FsNode {
    Directory(Directory),
    File(File),
}

impl FsNode {
    pub fn name(&self) -> &str {
        match self {
            Self::File(f) => f.name(),
            Self::Directory(d) => d.name(),
        }
    }

    pub fn parent(&self) -> Option<DirEntry> {
        match self {
            Self::File(f) => Some(f.parent()),
            Self::Directory(d) => d.parent(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
struct DirEntry(usize);

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

    fn get_node(&self, entry: DirEntry) -> &FsNode {
        &self.node_table[entry.0]
    }

    fn get_node_mut(&mut self, entry: DirEntry) -> &mut FsNode {
        &mut self.node_table[entry.0]
    }

    pub fn root(&self) -> DirEntry {
        DirEntry(0)
    }

    pub fn vacate(&mut self, entry: DirEntry) {
        self.vacancies.push(entry.0)
    }

    pub fn delete_recursive(&mut self, entry: DirEntry) -> Result<(), CannotDeleteRoot> {
        if entry == self.root() {
            return Err(CannotDeleteRoot);
        }

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

    fn collect_entries<'a>(&self, current: DirEntry, entries: &'a mut Vec<DirEntry>) {
        entries.push(current);
        if let FsNode::Directory(d) = self.get_node(current) {
            for entry in d.entries.iter() {
                self.collect_entries(*entry, entries);
            }
        }
    }
}
