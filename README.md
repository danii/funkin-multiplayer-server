Friday Night Funkin Multiplayer Server
======================================
This is the server software used for the [Friday Night Funkin multiplayer modification][client]. If you're looking to play the mod, go check out [this repository][client] instead! Otherwise, if you're interested in hosting a server for your friends, keep reading.

[client]: https://github.com/AlekEagle/Funkin

What This Does
--------------
This repository hosts the code for the multiplayer server. The multiplayer server keeps track of all players, and acts as a messenger and arbiter for the players. The server's role is rather crucial for the multiplayer modification, as without it there would be no way to actually connect and play.

A peer to peer model is not currently planned at all, and while there may be a slight chance we switch to a peer to peer model, chances are we won't.

Compiling
---------
To compile the multiplayer server, downloading the repository or cloning it with git is requried. If you don't know how to do that, just click on the little green button at the top that says `Code` <!-- maybe put an inline picture of it here? -->, and then click on `Download ZIP`. Once in your [Downloads folder][windows-downloads], right click it and click `Extract All`. A new folder should appear in your Downloads folder, which will contain a local instance of this repository.

Compiling the server requires at least version 1.50.0 of the Rust compiler. The typical way of installing the Rust compiler is via [rustup][rustup]. Linux users and users of other operating systems who use package managers (such as chocolatey or homebrew) might also have the opportunity of installing the latest Rust compiler via their package repositories.

Once you have [rustup][rustup] installed, you will want to open up a [command prompt][windows-prompt] or terminal, and run `rustup toolchain install stable` to install the latest stable Rust compiler.

After setting up the Rust compiler, compiling is a breeze, well, a rather long but smooth breeze. Change your command prompt's or terminal's directory to the folder you extracted, `cd %HOME%\Downloads\funkin-multiplayer-server`. Then just run `cargo build --release`, and once that's all done, you'll have a fresh binary in the `folder` within the new `target` folder within the folder you unzipped.

[windows-downloads]: %HOME%\Downloads
[windows-prompt]: %WINDIR%\System32\cmd.exe
[rustup]: https://rustup.rs/
