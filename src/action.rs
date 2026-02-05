use crate::Entries;

pub enum Action {
    None,
    Transit {
        id: String,
        cursor: Option<usize>,
    },
    TransitToTmpEntries {
        entries: Entries,
        cursor: Option<usize>,
    },
    Quit,
}
