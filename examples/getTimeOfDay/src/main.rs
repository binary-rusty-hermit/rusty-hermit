#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let sys_gettimeofday: u64 = 96;
        let ptr_timeval: *mut timeval = &mut timeval::default();
	let tz: usize = 0;

        // return value
        let mut ret_value: u64;

        unsafe{
                asm!(   "syscall",
                        in("rax") sys_gettimeofday,
                        in("rdi") ptr_timeval,
			in("rsi") tz,
                        lateout("rax") ret_value
                        );
            println!("Value returned: {}", ret_value);

            println!("tv_sec: {}", (*ptr_timeval).tv_sec);
            println!("tv_usec: {}", (*ptr_timeval).tv_usec);

        }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct timeval {
	pub tv_sec: i64,
	pub tv_usec: i64,
}

impl Default for timeval {
    fn default () -> timeval {
        timeval{tv_sec: 0 as i64, tv_usec : 0 as i64}
    }
}

