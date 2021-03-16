#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(asm)]

use std::env;
use std::ffi::CString;

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

/* Elf ABI */
const AT_NULL: u64 		=  0;
const AT_IGNORE: u64		=  1;
const AT_EXECFD: u64		=  2;
const AT_PHDR: u64		=  3;
const AT_PHENT: u64		=  4;
const AT_PHNUM: u64		=  5;
const AT_PAGESZ: u64		=  6;
const AT_BASE: u64		=  7;
const AT_FLAGS: u64		=  8;
const AT_ENTRY: u64		=  9;
const AT_NOTELF: u64		= 10;
const AT_UID: u64		= 11;
const AT_EUID: u64		= 12;
const AT_GID: u64		= 13;
const AT_EGID: u64		= 14;
const AT_PLATFORM: u64		= 15;
const AT_HWCAP: u64		= 16;
const AT_CLKTCK: u64		= 17;
const AT_DCACHEBSIZE: u64 	= 19;
const AT_ICACHEBSIZE: u64 	= 20;
const AT_UCACHEBSIZE: u64 	= 21;
const AT_SECURE: u64		= 23;
const AT_RANDOM: u64		= 25;
const AT_EXECFN: u64		= 31;
const AT_SYSINFO_EHDR: u64	= 33;
const AT_SYSINFO: u64		= 32;

// TODO Get values from boot info
// Hardcoded values.
const APP_SIZE: u64 = 0x6bca00;
const APP_START: usize = 0x0;
const APP_ENTRY_POINT: u64 = 0x400a50;
const APP_EHDR_PHOFF: u64 = 64;
const APP_EHDR_PHNUM: u16 = 6;
const APP_EHDR_PHENTSIZE: u16 = 56;

// Push ELF auxiliary vectors to the stack
fn push_auxv(at_type: u64, at_value: u64) {
	unsafe {
		asm!(
		     "push {0}",
                     "push {1}",
		     in(reg) at_value,
                     in(reg) at_type
		);
	}
}

fn main() {
	let auxv_platform = CString::new("x86_64").expect("CString::new failed");
	let auxv_platform_ptr = auxv_platform.as_ptr();

	println!("Binary loader");
/*
        for (env_var_key, env_var_value) in env::vars() {
            println!("env_vars: {}, {:?}", env_var_key, env_var_value);
        }
*/

	/* auxv */
	push_auxv(AT_NULL, 0x0);
	push_auxv(AT_IGNORE, 0x0);
	push_auxv(AT_EXECFD, 0x0);
	push_auxv(AT_PHDR, APP_START as u64 + APP_EHDR_PHOFF);
	push_auxv(AT_PHNUM, APP_EHDR_PHNUM as u64);
	push_auxv(AT_PHENT, APP_EHDR_PHENTSIZE as u64);
	push_auxv(AT_RANDOM, APP_START as u64);
	push_auxv(AT_BASE, 0x0);
	push_auxv(AT_SYSINFO_EHDR, 0x0);
	push_auxv(AT_SYSINFO, 0x0);
	push_auxv(AT_PAGESZ, 4096);
	push_auxv(AT_HWCAP, 0x0);
	push_auxv(AT_CLKTCK, 0x64); // mimic Linux
	push_auxv(AT_FLAGS, 0x0);
	push_auxv(AT_ENTRY, APP_ENTRY_POINT);
	push_auxv(AT_UID, 0x0);
	push_auxv(AT_EUID, 0x0);
	push_auxv(AT_GID, 0x0);
	push_auxv(AT_EGID, 0x0);
	push_auxv(AT_SECURE, 0x0);
	push_auxv(AT_SYSINFO, 0x0);
	push_auxv(AT_EXECFN, 0x0);
	push_auxv(AT_DCACHEBSIZE, 0x0);
	push_auxv(AT_ICACHEBSIZE, 0x0);
	push_auxv(AT_UCACHEBSIZE, 0x0);
	push_auxv(AT_NOTELF, 0x0);
	push_auxv(AT_PLATFORM, auxv_platform_ptr as u64);


	loop {}

	unsafe {
		asm!(
		    "xor rdx, rdx",
                    "mov rax, [{0}]",
                    "jmp rax",
		    in(reg) APP_ENTRY_POINT,
		    out("rdx") _,
                    out("rax") _
		);
	}

/*
	unsafe {
		asm!(
		    "xor rdx, rdx",
                    "mov {0}, {1}",
                    "jmp {0}",
                    out(reg) _,
                    in(reg) APP_ENTRY_POINT,
		    out("rdx") _
		);
	}
*/
}
