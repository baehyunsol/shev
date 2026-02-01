pub struct Entry<Data> {
    /// Users see this text in the list.
    pub title: String,

    /// Not visible to the users.
    pub data: Data,

    /// These are not visible to the users.
    /// But if you set these, users can jump to next/prev
    /// categories using a/s/z/x.
    pub category1: Option<String>,
    pub category2: Option<String>,

    /// This is visible to the user.
    /// Users can also jump to next/prev flag using space.
    pub flag: EntryFlag,
}

pub enum EntryFlag {
    None,
    Red,
    Green,
    Blue,
}
