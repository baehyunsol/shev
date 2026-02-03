# shev

A simple, opinionated gui framework in Rust.

It does only one thing: show a list of entries.

## Entry vs Entries vs EntriesMap

`Entries` is a list of `Entry`, and `EntriesMap` is a map of `Entries`. `EntriesMap` uses a string id to distinguish `Entries`.

At any moment, there're one `Entries` and one `Entry` that are selected. The left side of the interface shows the `Entry` that's selected, and the right side shows the `Entries`.
