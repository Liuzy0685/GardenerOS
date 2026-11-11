#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") args[0] => result,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
        );
    }
    result
}

fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len()],
    )
}

fn sys_exit(code: usize) -> ! {
    syscall(SYSCALL_EXIT, [code, 0, 0]);
    loop {}
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if sys_write(1, text.as_bytes()) == text.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

fn print(args: fmt::Arguments<'_>) {
    if Stdout.write_fmt(args).is_err() {
        sys_exit(1);
    }
}

macro_rules! println {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $arg)*))
    };
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello, world!");
    sys_exit(0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
