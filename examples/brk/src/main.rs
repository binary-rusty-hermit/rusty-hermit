#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let sys_read: u64 = 0;
        let sys_write : u64 = 1;
        let sys_open: u64 = 2;
        let sys_close: u64 = 3;
        let sys_fstat: u64 = 5;
	let sys_brk: u64 = 12;
        // return value
        let mut ret_value: i64;

	let mut addr: i64 = 0;

        unsafe{
		// BRK 0		
                asm!(   "syscall",
                        in("rax") sys_brk,
                        in("rdi") addr,
                        lateout("rax") ret_value);

		println!("End of heap is: {}", ret_value);

		addr = ret_value + 10;

                // BRK end of heap + 1;
                asm!(   "syscall",
                        in("rax") sys_brk,
                        in("rdi") addr,
                        lateout("rax") ret_value);
		
		 println!("End of heap is: {}", ret_value);

		addr = ret_value - 20;	
	
		// BRK end of heap - 20;
                asm!(   "syscall",
                        in("rax") sys_brk,
                        in("rdi") addr,
                        lateout("rax") ret_value);

		println!("End of heap is: {}", ret_value);

	}
}
