use crate::common::types::{PageNum, SlotNum};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RID {
    page_num: PageNum,
    solt_num: SlotNum,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    rid: RID,
    data: Vec<u8>,
}

impl Record {
    pub fn new(rid: RID, data: Vec<u8>) -> Record {
        Record {
            rid,
            data
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn set_rid(&mut self, rid: &RID) {
        self.rid.page_num = rid.page_num;
        self.rid.solt_num = rid.solt_num;
    }

    pub fn rid(&self) -> &RID {
        &self.rid
    }
}