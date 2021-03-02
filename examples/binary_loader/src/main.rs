#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
	// TODO Get entry point address
	println!("Hello World!");
/*
	unsafe {
		asm!("jmp {}", );
	}
*/
}
