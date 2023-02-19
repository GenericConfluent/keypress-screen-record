use chrono::prelude::*;
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
};
use std::fs;

fn capture() {
    let local_time = Local::now();

    // TODO: Option to output raw frames or write frames to temp dir instead and then process them
    // into video.
    let path = {
        let mut video_dir = dirs::video_dir()
            .expect("Could not find home or platform not linux, macos, or windows");
        video_dir.push("ksr");
        video_dir.push(local_time.format("%s%f").to_string());
        video_dir.set_extension("png");
        video_dir
    };

    let folder = path.parent().expect("Could not get outfile parent folder.");
    match folder.try_exists() {
        Ok(true) => {}
        Ok(false) => fs::create_dir_all(folder).expect("Could not create folders."),
        Err(error) => match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Need permission to write to video folder.")
            }
            other => panic!("{}", other),
        },
    }

    let path = format!("file://{}", path.to_str().unwrap());

    unsafe {
        match ffi::capture_to(path.as_ptr(), path.len() as libc::c_long, 0) {
            0 => {}
            n => eprintln!(
                "Something went wrong (CaptureResult variant {} zero indexed)",
                n
            ),
        }
    }
}

#[cfg(target_os = "macos")]
mod ffi {
    // These are dependent frameworks of the `src/capture.c` file. It just happens to be more
    // convenient to specify them here than in the build script. (I could not get things working
    // from the build script. I spent six hours.)
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {}

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {}

    #[link(name = "ImageIO", kind = "framework")]
    extern "C" {}

    #[link(name = "CoreServices", kind = "framework")]
    extern "C" {}

    extern "C" {
        pub(super) fn capture_to(
            path: *const u8,
            path_length: libc::c_long,
            screen: libc::size_t,
        ) -> libc::c_int;
    }
}

// TODO: Shutdown by hotkey rather than interrupt.
enum Message {
    Capture,
    Die,
}

fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || loop {
        match receiver.recv() {
            Ok(Message::Capture) => capture(),
            _ => break,
        }
    });

    let on_keypress = move |_event_proxy, event_type, event: &CGEvent| {
        match event_type {
            CGEventType::KeyDown => {
                sender
                    .send(Message::Capture)
                    .expect("Reciever dropped, could not sent message to capture thread.");
            }
            _ => {}
        }
        Some(event.clone())
    };

    let current = CFRunLoop::get_current();
    match CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::KeyDown, CGEventType::FlagsChanged],
        on_keypress,
    ) {
        Ok(tap) => unsafe {
            let loop_source = tap
                .mach_port
                .create_runloop_source(0)
                .expect("Some unknown problem occured");
            current.add_source(&loop_source, kCFRunLoopCommonModes);
            tap.enable();
            CFRunLoop::run_current();
        },
        Err(_) => eprintln!("Could not tap global input."),
    }
}
