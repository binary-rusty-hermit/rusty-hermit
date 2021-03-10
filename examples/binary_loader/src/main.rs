#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(asm)]

use std::env;

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
	// TODO Get entry point address
        let input: u64 = 0x400000;

	println!("Binary loader");
	unsafe {
		asm!(
                    "mov {0}, {1}",
                    "jmp {0}",
                    out(reg) _,
                    in(reg) input);
	}
}
