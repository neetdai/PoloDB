use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TransactionType {
    Read,
    Write,
}

pub(super) struct TransactionState {
    pub(super) ty: TransactionType,
    pub(super) offset_map: BTreeMap<u32, u64>,
    pub(super) frame_count: u32,
    pub(super) db_file_size: u64,
}

impl TransactionState {

    pub(super) fn new(ty: TransactionType, frame_count: u32, db_file_size: u64) -> TransactionState {
        TransactionState {
            ty,
            offset_map: BTreeMap::new(),
            frame_count,
            db_file_size,
        }
    }

}

