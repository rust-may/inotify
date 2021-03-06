# inotify-rs [![crates.io](https://img.shields.io/crates/v/inotify.svg)](https://crates.io/crates/inotify) [![Documentation](https://docs.rs/inotify/badge.svg)](https://docs.rs/inotify) [![Build Status](https://travis-ci.org/inotify-rs/inotify.svg?branch=master)](https://travis-ci.org/hannobraun/inotify-rs)

Idiomatic [inotify] wrapper for the [Rust programming language].

Using async io when used in coroutine context (no blocking worker threads).

When used in thread context, it's the same as the original project usage.

```Rust
#[macro_use]
extern crate may;
extern crate inotify;


use std::env;

use inotify::{
    event_mask,
    watch_mask,
    Inotify,
};


fn run_in_coroutine() {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    let current_dir = env::current_dir()
        .expect("Failed to determine current directory");

    inotify
        .add_watch(
            current_dir,
            watch_mask::MODIFY | watch_mask::CREATE | watch_mask::DELETE,
        )
        .expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(event_mask::CREATE) {
                if event.mask.contains(event_mask::ISDIR) {
                    println!("Directory created: {:?}", event.name);
                } else {
                    println!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(event_mask::DELETE) {
                if event.mask.contains(event_mask::ISDIR) {
                    println!("Directory deleted: {:?}", event.name);
                } else {
                    println!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(event_mask::MODIFY) {
                if event.mask.contains(event_mask::ISDIR) {
                    println!("Directory modified: {:?}", event.name);
                } else {
                    println!("File modified: {:?}", event.name);
                }
            }
        }
    }
}

fn main() {
    go!(run_in_coroutine).join().unwrap();
}
```


## Usage

Inlude it in your `Cargo.toml`:

```toml
[dependencies]
inotify = { git = "https://github.com/rust-may/inotify.git" }
```

Please refer to the [documentation] and the example above, for information on how to use it in your code.

Please note that inotify-rs is a relatively low-level wrapper around the original inotify API. And, of course, it is Linux-specific, just like inotify itself. If you are looking for a higher-level and platform-independent file system notification library, please consider [notify].

If you need to access inotify in a way that this wrapper doesn't support, consider using [inotify-sys] instead.


## Documentation

The most important piece of documentation for inotify-rs is the **[API reference]**, as it contains a thorough description of the complete API, as well as examples.

Additional examples can be found in the **[examples directory]**.

Please also make sure to read the **[inotify man page]**. Inotify use can be hard to get right, and this low-level wrapper won't protect you from all mistakes.


## License

Copyright (c) 2014-2017, Hanno Braun and contributors

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.


[inotify]: http://en.wikipedia.org/wiki/Inotify
[Rust programming language]: http://rust-lang.org/
[documentation]: https://docs.rs/inotify
[notify]: https://crates.io/crates/notify
[inotify-sys]: https://crates.io/crates/inotify-sys
[API reference]: https://docs.rs/inotify
[examples directory]: https://github.com/inotify-rs/inotify/tree/master/inotify/examples
[inotify man page]: http://man7.org/linux/man-pages/man7/inotify.7.html
