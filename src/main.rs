use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello, world!");

    let mut thread_handles = Vec::new();

    /*
    thread 'main' (2408710) panicked at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/functions.rs:131:29:
failed to spawn thread: Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

    /*
    With RUST_BACKTRACE=1 cargo run:
    thread 'main' (2444860) panicked at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/functions.rs:131:29:
failed to spawn thread: Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/result.rs:1867:5
   3: core::result::Result<T,E>::expect
             at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:1185:23
   4: std::thread::functions::spawn
             at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/functions.rs:131:29
   5: AsyncRustSecond::main
             at ./src/main.rs:15:22
   6: core::ops::function::FnOnce::call_once
             at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
     */

    /*
    With RUST_BACKTRACE=full cargo run:
    thread 'main' (2463552) panicked at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/functions.rs:131:29:
failed to spawn thread: Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }
stack backtrace:
   0:        0x10456afe0 - std[7d7552da0923e8b2]::backtrace_rs::backtrace::libunwind::trace
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:        0x10456afe0 - std[7d7552da0923e8b2]::backtrace_rs::backtrace::trace_unsynchronized::<std[7d7552da0923e8b2]::sys::backtrace::_print_fmt::{closure#1}>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:        0x10456afe0 - std[7d7552da0923e8b2]::sys::backtrace::_print_fmt
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/sys/backtrace.rs:74:9
   3:        0x10456afe0 - <<std[7d7552da0923e8b2]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[4b39a0a778b8475a]::fmt::Display>::fmt
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/sys/backtrace.rs:44:26
   4:        0x104578d38 - <core[4b39a0a778b8475a]::fmt::rt::Argument>::fmt
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/fmt/rt.rs:152:76
   5:        0x104578d38 - core[4b39a0a778b8475a]::fmt::write
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/fmt/mod.rs:1687:22
   6:        0x10456e768 - std[7d7552da0923e8b2]::io::default_write_fmt::<std[7d7552da0923e8b2]::sys::stdio::unix::Stderr>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/io/mod.rs:621:11
   7:        0x10456e768 - <std[7d7552da0923e8b2]::sys::stdio::unix::Stderr as std[7d7552da0923e8b2]::io::Write>::write_fmt
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/io/mod.rs:1976:13
   8:        0x1045589ac - <std[7d7552da0923e8b2]::sys::backtrace::BacktraceLock>::print
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/sys/backtrace.rs:47:25
   9:        0x1045589ac - std[7d7552da0923e8b2]::panicking::default_hook::{closure#0}
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:292:27
  10:        0x1045655b0 - std[7d7552da0923e8b2]::panicking::default_hook
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:319:9
  11:        0x1045658d8 - std[7d7552da0923e8b2]::panicking::panic_with_hook
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:825:13
  12:        0x104558a58 - std[7d7552da0923e8b2]::panicking::panic_handler::{closure#0}
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:698:13
  13:        0x10454e414 - std[7d7552da0923e8b2]::sys::backtrace::__rust_end_short_backtrace::<std[7d7552da0923e8b2]::panicking::panic_handler::{closure#0}, !>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/sys/backtrace.rs:182:18
  14:        0x104559128 - __rustc[8068f81614cfe5c]::rust_begin_unwind
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:689:5
  15:        0x104580674 - core[4b39a0a778b8475a]::panicking::panic_fmt
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/panicking.rs:80:14
  16:        0x1045804a4 - core[4b39a0a778b8475a]::result::unwrap_failed
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/result.rs:1867:5
  17:        0x104544fa4 - core::result::Result<T,E>::expect::hd31c46cf5690ab75
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:1185:23
  18:        0x1045452c4 - std::thread::functions::spawn::h1b4cc82320e0a7cb
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/functions.rs:131:29
  19:        0x104546978 - AsyncRustSecond::main::hc2049e1ede14710a
                               at /Users/sridharvenkat/Documents/rust/AsyncRustSecond/src/main.rs:36:22
  20:        0x104546d54 - core::ops::function::FnOnce::call_once::hf2253811951ba633
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  21:        0x1045464ac - std::sys::backtrace::__rust_begin_short_backtrace::hed0b4e58dedb7fd9
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
  22:        0x104544c88 - std::rt::lang_start::{{closure}}::h48a1de645831be1b
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  23:        0x104564d18 - <&dyn core[4b39a0a778b8475a]::ops::function::Fn<(), Output = i32> + core[4b39a0a778b8475a]::marker::Sync + core[4b39a0a778b8475a]::panic::unwind_safe::RefUnwindSafe as core[4b39a0a778b8475a]::ops::function::FnOnce<()>>::call_once
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/core/src/ops/function.rs:287:21
  24:        0x104564d18 - std[7d7552da0923e8b2]::panicking::catch_unwind::do_call::<&dyn core[4b39a0a778b8475a]::ops::function::Fn<(), Output = i32> + core[4b39a0a778b8475a]::marker::Sync + core[4b39a0a778b8475a]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:581:40
  25:        0x104564d18 - std[7d7552da0923e8b2]::panicking::catch_unwind::<i32, &dyn core[4b39a0a778b8475a]::ops::function::Fn<(), Output = i32> + core[4b39a0a778b8475a]::marker::Sync + core[4b39a0a778b8475a]::panic::unwind_safe::RefUnwindSafe>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:544:19
  26:        0x104564d18 - std[7d7552da0923e8b2]::panic::catch_unwind::<&dyn core[4b39a0a778b8475a]::ops::function::Fn<(), Output = i32> + core[4b39a0a778b8475a]::marker::Sync + core[4b39a0a778b8475a]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panic.rs:359:14
  27:        0x104564d18 - std[7d7552da0923e8b2]::rt::lang_start_internal::{closure#0}
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/rt.rs:175:24
  28:        0x104564d18 - std[7d7552da0923e8b2]::panicking::catch_unwind::do_call::<std[7d7552da0923e8b2]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:581:40
  29:        0x104564d18 - std[7d7552da0923e8b2]::panicking::catch_unwind::<isize, std[7d7552da0923e8b2]::rt::lang_start_internal::{closure#0}>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panicking.rs:544:19
  30:        0x104564d18 - std[7d7552da0923e8b2]::panic::catch_unwind::<std[7d7552da0923e8b2]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/panic.rs:359:14
  31:        0x104564d18 - std[7d7552da0923e8b2]::rt::lang_start_internal
                               at /rustc/ac68faa20c58cbccd01ee7208bf3b6e93a7d7f96/library/std/src/rt.rs:171:5
  32:        0x104544c60 - std::rt::lang_start::hae27fddc5ca774a4
                               at /Users/sridharvenkat/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  33:        0x104546b58 - _main
     */
    for i in 0..100000 {
        let handle = thread::spawn(move || {
            let result = add(i, i + 1);
            println!("Result of adding {} and {} is {}", i, i + 1, result);
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    thread::sleep(Duration::from_secs(1));
    println!("Done adding {} and {}", a, b);
    a + b
}
