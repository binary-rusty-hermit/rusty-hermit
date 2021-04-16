#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let linkpath = "mylink\0";
	let path_ptr = linkpath.as_ptr();
        let target = "      ";

        let target_ptr = target.as_ptr();
	let length = target.len();
        let sys_readlink : u64 = 89;

        // return value
        let mut ret_value: u64;

        unsafe{
                asm!(   "syscall",
                        in("rax") sys_readlink,
                        in("rdi") path_ptr,
                        in("rsi") target_ptr,
                        in("rdx") length,
                        lateout("rax") ret_value
                        );

	println!("target is: {} \nvalue returned: {}", target, ret_value);



	}

}
