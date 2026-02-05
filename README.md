# shev

`shev` is a very very opinionated gui framework for me.

When I write software, I also write automated test scripts. The test script (usually in Python) runs a test suite, which consists of multiple test cases. The test script dumps the test results to files (usually in json). It usually creates a file per test case. So, when I run a test suite, I get hundreds of result files. I run a test suite everytime I make change, so there are results of multiple test suites.

I need a GUI interface that 1) I can see the results of test cases of a test suite, 2) I can compare the results of a test case across multiple test suites, 3) I can easily browse this 2d (suite x case) space.

I found myself writing the "test-result-viewer" over and over. So, I decided to write a framework that helps me create a "test-result-viewer".

## Entry vs Entries vs EntriesMap

`Entry`, `Entries` and `EntriesMap` are very different and very important concepts in shev.

`Entries` is a list of `Entry`, and `EntriesMap` is a map of `Entries`. `EntriesMap` uses a string id to distinguish `Entries`.

At any moment, there're one `Entries` and one `Entry` that are selected. The left side of the interface shows the `Entry` that's selected, and the right side shows the `Entries`.

Usually, `Entry` is a result of a test case. It has a flag (success/fail), and prettified result (e.g. stderr). `Entries` is a list of `Entry`s, so `Entries` is a test suite. So, in a view, you see a result of a test case on your left and test cases of a test suite on your right.

`EntriesMap` is a map of `Entries`, so you store test suites in an `EntriesMap`. You might want to split a test suite into multiple `Entries`. Also, you might want to add a meta-`Entries` which has a list of test suites (each suite is an `Entry` in the `Entries`).

## EntryFlag vs EntryState

For a "test-result-viewer", it's nice to have a flag that indicates whether a test case is successful or not. `EntryFlag` is for that. When you create an `Entry` for a test case, you might want to set `entry.flag = EntryFlag::Green` when the case is successful and `entry.flag = EntryFlag::Red` when it's not. You can't mutate `EntryFlag` in the GUI.

It is likely that an entry has multiple views. For example, if an `Entry` is a result of running `test-1.py`, you want a view for the content of `test-1.py` and stdout/stderr of the script. In this case, you check `EntryState` inside `render_canvas` function, and return different contents depending on `EntryState`. Users can change `EntryState` in GUI using M key.

## Known issues

There's a [bug](https://github.com/not-fl3/macroquad/issues/980) in macroquad. So you should NEVER press CMD key if you're using MacOS.
