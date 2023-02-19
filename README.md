# Installation
There is no convenient way to install ksr to the system at the moment. So
easiest way to get started is to clone and build with:
```bash
git clone https://github.com/GenericConfluent/keypress-screen-record.git
cd keypress-screen-record
cargo run
```

# Use
Only supports macos at the moment and the functionality is limited. The only
thing you can do is run the program and then visit `~/Movies/Ksr/` to see the
recorded screenshots. 

If this is something you might be interested in using feel free to submit pull
requests with changes. You are especially welcome if you're looking to implement 
screenshot behaviour on other platforms.

# Why is `capture_to` written in C?
1. Rust screenshot libraries worked too slowly for my liking.
   [screenshot**s**-rs](https://github.com/nashaofu/screenshots-rs) was
   brutally slow and you actually couldn't take shots fast enough to capture
   the ~10 chars/s a person was typing. Though it was more feature complete and
   is probably better suited for people just looking to take a single
   screenshot. [screenshot-rs](https://github.com/alexchandel/screenshot-rs)
   was quick but transfering image data to the image crate and then writing it
   took longer than I would have liked, also only forks of the crate will
   build.
2. I couldn't get the bindings working in rust and it seemed like less work to
   write the code in C and set up ffi than to write out the bindings for the
   necessary functions myself.

