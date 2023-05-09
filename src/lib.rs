#![cfg(windows)]

use utfx::U16CString;
use windows_sys::Win32::{
    System::{Environment::GetCommandLineW, Memory::LocalFree},
    UI::Shell::CommandLineToArgvW,
};

pub struct Args {
    raw_args: *mut *mut u16,
    cur: i32,
    len: i32,
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.len {
            return None;
        }

        let poop = unsafe { *self.raw_args.add(self.cur as usize) };

        let u16cs = unsafe { U16CString::from_ptr_str(poop) };
        let out = u16cs.to_string_lossy();

        self.cur += 1;
        Some(out)
    }
}

impl Drop for Args {
    fn drop(&mut self) {
        unsafe { LocalFree(self.raw_args as _) };
    }
}

/// Everyone's favourite `CommandLineToArgvW` but safe.
pub fn command_line_to_argv(input: Option<&str>) -> Args {
    let input = input.map(|x| {
        x.encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>()
    });
    let mut len = 0i32;
    let raw_args = unsafe {
        CommandLineToArgvW(
            input
                .map(|x| x.as_ptr())
                .unwrap_or_else(|| GetCommandLineW()),
            &mut len,
        )
    };
    Args {
        raw_args,
        len,
        cur: 0,
    }
}

#[test]
fn lol() {
    println!("{:?}", command_line_to_argv(None).collect::<Vec<_>>());
}