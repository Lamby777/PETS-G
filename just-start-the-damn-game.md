# Setup

"Just play the damn game! What's all this setup for?"

## Intro

Alright, so I've found out that most people who ask me where they can
play this game AREN'T exactly massive computer nerds, and end up wasting like
20-30 minutes on the pain-in-the-ass side quest known as installing the rust
toolchain and visual studio stuff. Here's a "for noobs" guide on all the stuff
you gotta go through to set it up. This is for Windows, because Mac support is
questionable rn, and Linux users only really need to run a couple shell commands
to get it done.

In this guide, when I say you need to open PowerShell in a specific folder,
you can do that by holding Shift and right-clicking the empty space inside that
folder, then clicking "Open PowerShell window here"

---

## Get the stuff

These are the tools that devs will need in order to playtest the game.

They aren't necessary if the game is exported through the editor, but that takes
time and also requires you to trust me not to sneak suspicious code into the file
before sending it to you. With this method, you can try out the latest version on
your own computer right now!

---

### Git

To work with branches and stuff, you'll need Git. You could just download the repo
as a zip, but having this will help you get updates or even contribute, so having
it doesn't hurt.

Get [Git for Windows](https://git-scm.com/download/win) here.

### Rust Toolchain

This is needed in order to build the library.

Get it [here](https://rustup.rs/). (ignore the stuff about WSL, don't run that)

Follow along in the installer window. If it asks you about installing a linker and
Windows libraries, pick `Quick install via the Visual Studio Community installer`

### Godot

And finally, we'll need the game engine that P/E/T/S is built on.
Download it [here](https://godotengine.org/download/windows/).

---

## Get the code and build it

Still with me? Good. The rest is pretty easy.

Open a PowerShell window. Then, paste this stuff in and hit enter:

```ps1
cd ~\Desktop
git clone https://github.com/Lamby777/PETS-G
cd PETS-G\pets-lib
cargo build
```

If there are issues here, it's usually not your fault. Send me a screenshot.

---

## Open it in Godot

There should be a folder called `PETS-G` on your desktop now.

You're now ready to open the game! Open Godot, and click "Import" in the top
left. Navigate to `<desktop>\PETS-G\pets-gd` and pick the `project.godot` file.

To play the game, click the play button in the top right, or press F5.
