use crate::Entry;

pub struct Filter {
    pub name: String,
    pub cond: fn(&Entry) -> bool,
}
