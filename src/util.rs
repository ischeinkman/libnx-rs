use std::panic;
use std::process::exit;
use std::thread;

/// Makes panics print to stdout and exit with code 0, avoiding causing a reboot.                                                       
pub fn no_crash_panic() {
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        old_hook(info);
        let thread = thread::current();
        let name = thread.name().unwrap_or("<unnamed>");
        println!("thread '{}' {}", name, info);
        exit(0);
    }));
}
