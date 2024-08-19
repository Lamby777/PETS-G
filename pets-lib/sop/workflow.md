# Workflow

Some of this information may seem obvious if you've been using Godot for a while,
but I'll include it anyway since I didn't know for a while.

## GDExtension

You're probably gonna want to spend most of your time in the `pets-lib` folder.
This is where the Rust side of the codebase is in. The `pets-gd` folder has the
Godot project, but that's edited with the actual Godot editor. Plus, there are
symlinks to most of the important stuff in `pets-lib` anyway.

When you finish making changes, build the library with `cargo run` and start the
game. Pretty self-explanatory from there. For quick edits, `cargo-watch` is your
friend.

## Starting from CLI

There's a quickstart script in `pets-lib` for my CLI text editor bois. The
`./run` script just opens the main scene using the `godot` command with whatever
arguments you give. This is pretty nice for testing purposes, but don't worry
about reloading the editor each time... Hot reloads are now supported by Godot!
You should only have to reload it when making a new class or changing fields on
a class, or anything along those lines.

Refer to
[this wiki page](https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html)
for arguments n stuff to pass into these scripts. One useful argument is `-e`,
which opens the editor instead of running the scene as a game.
