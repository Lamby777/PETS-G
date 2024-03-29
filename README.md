# P/E/T/S (Rewritten)

Rewriting my game I started back in 2019, except actually finishing it this time
because I picked a good game engine that doesn't make basic programming and
control of your own hardware overly complicated... and is also open-source! <3

---

## Somewhat Useful Links

- [Soundtrack](https://www.youtube.com/playlist?list=PLxLRTqK8yZMO14zFG12650hGkPOZYV_2p)
- [Promo page](https://sparklet.org/pets)
- [Dialogue Toolkit](https://github.com/Lamby777/dialogical)
- [Discord server](https://discord.gg/xEWa6Kwcad)

---

On Godot version 4.2.1

Using [godot-rust/gdext](https://github.com/godot-rust/gdextension) for most of
the stuff, because of the type system. Prefer GDScript for things that probably
won't need to interact with Rust code, and are simpler to just write as GDScript
scripts.

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

That's it! The dialogue toolkit is now called directly as a build dependency, so
you no longer need to have `dg` installed system-wide.

---

## (Recommended...?) Workflow

You're probably gonna want to spend most of your time in the `pets-lib` folder.
This is where the Rust side of the codebase is in. The `pets-gd` folder has the
Godot project, but that's edited with the actual Godot editor.

There are 2 quickstart scripts in `pets-lib` for my CLI text editor bois. The
`run` script just opens the main scene with whatever arguments you give. The
`battle` one similarly just opens the battle scene. This is pretty nice for
testing purposes, but don't worry about reloading each time... Hot reloads are
now supported.

Refer to
[this wiki page](https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html)
for arguments n stuff to pass into these scripts. One useful argument is `-e`,
which opens the editor instead of running the scene as a game.

When you finish making changes, build the library and start the game. Pretty
self-explanatory from there.
