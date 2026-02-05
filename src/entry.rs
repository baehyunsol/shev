use crate::Graphic;

pub struct Entry {
    /// Users see this name in the side-bar.
    pub name: String,

    /// There's no way for the users to directly see this content.
    /// Instead, you have to implement `render_canvas` function
    /// to render a content of an entry.
    /// The most straight forward way is to use `Graphic::text_box` function.
    pub content: Option<String>,

    /// TODO: regex pattern matching
    ///
    /// If an `Entry` has a `.search_corpus`, shev's regex search engine will
    /// use this corpus instead of `.content`.
    pub search_corpus: Option<String>,

    /// The user can filter `Entry`s by categories. (WIP)
    pub categories: Vec<String>,

    /// `transition1` and `transition2` have ids of another `Entries`.
    /// The user can transit to this `Entries` with K/L key.
    pub transition1: Option<Transition>,
    pub transition2: Option<Transition>,

    /// This is visible to the user, in the side-bar.
    /// Users can also jump to next/prev entry with the same flag using Space key.
    /// This is immutable. The users cannot change flag. For mutable states,
    /// use `EntryState`.
    ///
    /// If you're using shev to render a result of a test suite, you can use this to
    /// indicate whether a test case is successful.
    ///
    /// TODO: filter by flag
    pub flag: EntryFlag,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            name: String::new(),
            content: None,
            search_corpus: None,
            categories: vec![],
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

    /// How many states an entry can have.
    /// You might want to implement multiple views for an entry.
    /// Let's say you want 3 views (view A, view B and view C). Then you set
    /// this value to 3, and make `render_canvas` render different views
    /// according to `EntryState` value. The user can change `EntryState`
    /// by pressing M Key.
    pub entry_state_count: u32,

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
    /// The user can change `EntryState` by pressing M key.
    pub render_canvas: fn(&Entry, EntryState) -> Result<Vec<Graphic>, String>,

    /// If you set this, you can dump extra message to the top-bar.
    pub render_top_bar_extra_message: Option<fn(&Entry, EntryState) -> Option<String>>,
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
            entry_state_count: 1,
            transition: None,
            render_canvas: |_, _| Ok(vec![]),
            render_top_bar_extra_message: None,
        }
    }
}

impl std::ops::Index<usize> for Entries {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntryState(pub u32);

#[derive(Clone, Debug)]
pub struct Transition {
    pub id: String,
    pub description: Option<String>,
}
