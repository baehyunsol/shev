use crate::Graphic;

pub struct Entry {
    /// Users see this text in the side-bar.
    pub side_bar_title: String,
    pub top_bar_title: Option<String>,

    /// There's no way for the users to directly see this content.
    /// Instead, you have to implement `render_canvas` function
    /// to render a content of an entry.
    /// The most straight forward way is to use `Graphic::text_box` function.
    pub content: Option<String>,

    /// Users can see the extra content with C key, if exists.
    pub extra_content: Option<String>,

    /// `category1` and `category2` are not visible to the users.
    /// But if you set these, users can jump to next/prev
    /// categories using Ctrl(+Shift)+up/down.
    pub category1: Option<String>,
    pub category2: Option<String>,

    /// `transition1` and `transition2` have ids of another `Entries`.
    /// The user can transit to this `Entries` with K/L key.
    pub transition1: Option<Transition>,
    pub transition2: Option<Transition>,

    /// This is visible to the user, in the side-bar.
    /// Users can also jump to next/prev entry with the same flag using Space key.
    /// This is immutable. The users cannot change flag. For mutable states,
    /// use `EntryState`.
    pub flag: EntryFlag,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            side_bar_title: String::new(),
            top_bar_title: None,
            content: None,
            extra_content: None,
            category1: None,
            category2: None,
            transition1: None,
            transition2: None,
            flag: EntryFlag::None,
        }
    }
}

pub struct Entries {
    pub id: String,
    pub title: Option<String>,
    pub entries: Vec<Entry>,

    /// This has an id of another `Entries`.
    /// The user can transit to this `Entries` with J key.
    pub transition: Option<Transition>,

    /// The engine will use this function to render the currently selected `Entry`.
    /// The canvas size is always 900x600. If the graphic goes out of canvas,
    /// the user has to use WASD to move the camera.
    ///
    /// This function is cached. It's called only when a new `Entry` is selected or
    /// `EntryState` is changed.
    ///
    /// The user can change `EntryState` by pressing M key. The state changes in
    /// `None` -> `Red` -> `Green` -> `Blue` order, and it's set to `None` when
    /// a new `Entry` is selected.
    pub render_canvas: fn(&Entry, EntryState) -> Result<Vec<Graphic>, String>,
}

impl Entries {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item=&Entry> {
        self.entries.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index)
    }
}

impl Default for Entries {
    fn default() -> Entries {
        Entries {
            id: String::new(),
            title: None,
            entries: vec![],
            transition: None,
            render_canvas: |_, _| Ok(vec![]),
        }
    }
}

impl std::ops::Index<usize> for Entries {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryFlag {
    None,
    Red,
    Green,
    Blue,
}

impl EntryFlag {
    pub fn is_some(&self) -> bool {
        !matches!(self, EntryFlag::None)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryState {
    None,
    Red,
    Green,
    Blue,
}

impl EntryState {
    #[must_use = "method returns a new value and does not mutate the original value"]
    pub fn next(&self) -> EntryState {
        match self {
            EntryState::None => EntryState::Red,
            EntryState::Red => EntryState::Green,
            EntryState::Green => EntryState::Blue,
            EntryState::Blue => EntryState::None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub id: String,
    pub description: Option<String>,
}
