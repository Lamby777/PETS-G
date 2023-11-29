# P/E/T/S (Rewritten)

Rewriting my game I started back in 2019, except actually finishing it this time
because I picked a good game engine that doesn't make basic programming and
control of your own hardware overly complicated... and is also open-source! <3

---

## Somewhat Useful Links

- [Soundtrack](https://www.youtube.com/playlist?list=PLxLRTqK8yZMO14zFG12650hGkPOZYV_2p)
- [Promo page](https://sparklet.org/pets)
- [Discord server](https://discord.gg/xEWa6Kwcad)

---

Using [godot-rust/gdext](https://github.com/godot-rust/gdextension) for complex
parts because compile-time type safety go brrrr. Prefer GDScript for the simple
parts of the game's code, though.

Using Godot version 4.1.2.

Godot 4.2 is coming out soon, and I don't see any reason not to update yet, so
unless some major shit goes down in terms of compatibility, I'll probably update
to that too. (Updating from 4.0 -> 4.1 was kind of a massive waste of my time)

---

This game is... well, obviously... free and open-source software! Yay! 🎉

Many of my favorite games are either entirely free (no microtransactions) OR
open-source / respect your digital freedoms. Be the change you wish to see in
the world, right?

---

## Setting Up

First, make sure you're using the `nightly` channel for Rust. It **WILL NOT
COMPILE ON STABLE** due to use of experimental Rust features!

Then, go download the correct version of Godot Engine... with whatever method is
best for your environment. Add it to your `PATH` as well if you want the "quick
launch" scripts mentioned below to work.

Then, install `dg`, a crate written for managing and compiling dialogue /
interaction files for this game.

```
cargo install dialogical
```

... and you should be ready to go!

---

## (Recommended...?) Workflow

You're probably gonna want to spend most of your time in the `pets-lib` folder.
This is where the Rust side of the codebase is in. The `pets-gd` folder has the
Godot project, but that's edited with the actual Godot editor.

There are 2 quickstart scripts in `pets-lib` for my CLI text editor bois. The
`run` script just opens the main scene with whatever arguments you give. The
`battle` one similarly just opens the battle scene. This is pretty nice for
testing purposes, but I think hot reloading GDExtension Rust libraries might
have support now...? (Not entirely sure... I changed a string once and it
updated on next startup, but idk about entire classes/nodes. Those probably need
restarts.)

Refer to
[this wiki page](https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html)
for arguments n stuff to pass into these scripts. One useful argument is `-e`,
which opens the editor instead of running the scene as a game.

When you finish making changes, build the library and start the game. Pretty
self-explanatory from there.
