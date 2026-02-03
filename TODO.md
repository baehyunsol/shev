# 6. reload entries

I want an operation that 1) creates `entries_map` from scratch, 2) reset `RenderCache`, but 3) doesn't update `state`.

# 5. better caching

1. implement LRU cache for `TextureCache`
2. use `HashMap<(usize, EntryState), Vec<Graphic>>` for `RenderCache`

# 4. colored `text_box`

for syntax highlighting

# 3. portrait UI

landscape is 1080x720, how about 720x1080?

Then, we need 4 layouts: landscape/portrait, wide_side_bar/narrow_side_bar

# 2. transition between `Entries`

The transitions are recorded in `Entry` and `Entries`.

# 1. auto line breaks in `text_box`
