#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let sys_uname: u64 = 63;
	let ptr_utsname: *mut Utsname = &mut Utsname::default(); 	

        // return value
        let mut ret_value: u64;

        unsafe{
                asm!(   "syscall",
                        in("rax") sys_uname,
                        in("rdi") ptr_utsname,
                        lateout("rax") ret_value
                        );
	    println!("Value returned: {}", ret_value);

            println!("sysname: {}", String::from_utf8_lossy(&(*ptr_utsname).sysname));
            println!("nodename: {}", String::from_utf8_lossy(&(*ptr_utsname).nodename));
            println!("release: {}", String::from_utf8_lossy(&(*ptr_utsname).release));
            println!("version: {}", String::from_utf8_lossy(&(*ptr_utsname).version));
            println!("machine: {}", String::from_utf8_lossy(&(*ptr_utsname).machine));
            println!("domainname: {}", String::from_utf8_lossy(&(*ptr_utsname).domainname));

	}
}

#[repr(C)]
pub struct Utsname {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

impl Default for Utsname {
    fn default () -> Utsname {
        Utsname{sysname: [0; 65], nodename: [0; 65], release: [0; 65], version: [0; 65], machine: [0; 65], domainname: [0; 65]}
    }
}
