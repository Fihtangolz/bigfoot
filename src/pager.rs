use std::vec::Vec;
use std::fs as fs;
use std::io::SeekFrom;
use std::io::prelude::*;

enum RowType {
    INT,
    STR,
} 

impl RowType {
    pub fn mem_size(&self) -> usize {
        match self {
            INT => 8,
            STR => 120,
        }
    }
}

pub struct CollumnDescriptor {
    name: String,
    ty: RowType,
}

struct Schema {
    collumns_descriptors: Vec<CollumnDescriptor>,
}

impl Schema {
    pub fn row_mem_size(&self) -> usize {
        self.collumns_descriptors.iter().fold(0, |acc, desc| acc + desc.ty.mem_size())
    }
}

pub struct Table {
    schema: Schema,
    row_number: usize,
    name: String,
    file: fs::File,
}

impl Table {
    pub fn new(name: &str) -> Self {
        let file = fs::File::create(name).unwrap();
        Self {
            schema: Schema {
                collumns_descriptors: vec![],
            },
            row_number: 0,
            name: name.to_string(),
            file: file,
        }
    }
}

