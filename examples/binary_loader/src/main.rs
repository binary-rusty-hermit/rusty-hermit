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
#[inline(always)]
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

	// Get the number of command line args and env vars
	let argc: usize = env::args().count() - 1;
        let envc: usize = env::vars().count();

        println!("argc: {}\nenvc: {}", argc, envc);

	// Create vector of CString pointers to env vars.

	//let env_vars: Vec<String> = env::vars().map(|(key, val)| format!("{}={}", key, val)).collect();
	//let env_vars: Vec<_> = env::vars().map(|env| CString::new(env.as_str()).unwrap()).collect();
	let env_vars: Vec<_> = env::vars()
				.map(|(key, val)| CString::new(format!("{}={}", key, val)).unwrap())
				.collect();

        let mut env_vars_ptr: Vec<_> = env_vars.iter()
				.map(|env| env.as_ptr())
				.collect();

	env_vars_ptr.push(std::ptr::null());


	// Create vector of CString pointers to argv elements.
	let argv: Vec<_> = env::args()
				.map(|arg| CString::new(arg.as_str()).unwrap())
				.collect();

        let mut argv_ptr: Vec<_> = argv.iter()
				.map(|arg| arg.as_ptr())
				.collect();

	argv_ptr.push(std::ptr::null());

	for i in 0..envc {
		println!("envv[{}]: {:?}", i, env_vars[i]);
	}
	println!("env_vars_ptr: {:?}", env_vars_ptr);
	println!("argv_ptr: {:?}", argv_ptr);
	for env_vp in env_vars_ptr.iter().rev() {
		println!("env_vp: {:?}", env_vp);
	}

	println!("Binary loader");
	//println!("{}", envc);
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


	// Push env var pointers to the stack in reverse order. Starting with null.
	for env_p in env_vars_ptr.iter().rev() {
		unsafe {
			asm!(
			    "push {0}",
			    in(reg) *env_p as u64
			);
		}
	}

/*
	for i in (0..envc).rev() {
		unsafe {
			asm!(
			    "push {0}",
			    in(reg) env_vars_ptr[i]
			);
		}
	}
*/
	//loop {}

	// Push argv pointers to the stack in reverse order. Starting with null.
	for argv_p in argv_ptr.iter().rev() {
		unsafe {
			asm!(
			    "push {0}",
			    in(reg) *argv_p as u64
			);
		}
	}

	// Push argc
	unsafe {
		asm!(
		    "push {0}",
		    in(reg) argc as u64
		);
	}


	loop {}
	// Clear value in rdx and jump to entry point.
	unsafe {
		asm!(
		    "xor rdx, rdx",
                    "mov rax, [{0}]",
                    "jmp rax",
		    in(reg) APP_ENTRY_POINT
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
