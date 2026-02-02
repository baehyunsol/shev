pub struct Entry {
    /// Users see this text in the side-bar.
    pub side_bar_title: String,
    pub top_bar_title: Option<String>,

    /// These are not visible to the users.
    /// But if you set these, users can jump to next/prev
    /// categories using Ctrl(+Shift)+up/down.
    pub category1: Option<String>,
    pub category2: Option<String>,

    /// This is visible to the user.
    /// Users can also jump to next/prev flag using space.
    pub flag: EntryFlag,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryFlag {
    None,
    Red,
    Green,
    Blue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryState {
    None,
    Red,
    Green,
}
