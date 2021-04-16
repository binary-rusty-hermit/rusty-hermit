#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let message = "Hello World!\n";

        let length = message.len();
        let filename = "Hello_world.txt\0"; // Filename has to end with \0
        let mut fd : i32 = 0;
        let message_ptr = message.as_ptr();
        let name_ptr = filename.as_ptr();

        let flags: i32 = 0o0100 | 0o0002; // flags to create a file if it doesn't exist and read an$
        let mode: i32 = 777;            // Permissions of the file upon creation: read, write, exec$

        let sys_read: u64 = 0;
        let sys_write : u64 = 1;
        let sys_open: u64 = 2;
        let sys_close: u64 = 3;
	let sys_fstat: u64 = 5;
        // return value
        let mut ret_value: u64;

	let mut st_ptr = &mut stat::default();

        unsafe{
                // Open
                asm!(   "syscall",
                        in("rax") sys_open,
                        in("rdi") name_ptr,
                        in("rsi") flags,
                        in("rdx") mode,
                        lateout("rax") fd);


		// Write
                asm!(   "syscall",
                	in("rax") sys_write,
                        in("rdi") fd,
                        in("rsi") message_ptr,
                        in("rdx") length,
                        lateout("rax") ret_value);

          	// Close
                asm!(   "syscall",
                in("rax") sys_close,
                in("rdi") fd);

               	// Open
                asm!(   "syscall",
                	in("rax") sys_open,
                        in("rdi") name_ptr,
                        in("rsi") flags,
                        in("rdx") mode,
                        lateout("rax") fd);

           	// Fstat
                asm!(   "syscall",
                 	in("rax") sys_fstat,
                     	in("rdi") fd,
                     	in("rsi") st_ptr,
			lateout("rax") ret_value);

		println!("Value returned from fstat: {}", ret_value);
                println!("dev: {}\nino: {}\nlink: {}\nmode: {}\nuid: {}\ngid: {}\nrdev: {}\nsize {}\nblksize: {}\nblocks: {}",
                        st_ptr.st_dev, st_ptr.st_ino, st_ptr.st_nlink, st_ptr.st_mode, st_ptr.st_uid, st_ptr.st_gid, st_ptr.st_rdev, st_ptr.st_size, st_ptr.st_blksize, st_ptr.st_blocks);
                println!("atim tv_sec: {} tv_nsec: {}\nmtim tv_sec: {} tv_nsec: {}\nctim tv_sec: {} tv_nsec: {}",
                        st_ptr.st_atim.tv_sec, st_ptr.st_atim.tv_nsec, st_ptr.st_mtim.tv_sec, st_ptr.st_mtim.tv_nsec, st_ptr.st_ctim.tv_sec, st_ptr.st_ctim.tv_nsec);

           	// Close
            	asm!(   "syscall",
                     	in("rax") sys_close,
                    	in("rdi") fd);

		println!("We are done!");
        } // unsafe
} // main

#[repr(C)]
#[derive(Debug, Default)]
struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

#[repr(C)]
#[derive(Debug, Default)]
struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i32,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    // some fields omitted
}


