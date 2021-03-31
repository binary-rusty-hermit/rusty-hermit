#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let message = "Hello World!";
        let tmp = "             ";
        let tmp_ptr = tmp.as_ptr();

        let length = message.len();
        let filename = "Hello_world.txt\0"; // Filename has to end with \0
        let mut fd : i32 = 0;
        let message_ptr = message.as_ptr();
        let name_ptr = filename.as_ptr();

        let flags: i32 = 0o0100 | 0o0002; // flags to create a file if it doesn't exist and read and write operations
        let mode: i32 = 777;            // Permissions of the file upon creation: read, write, execute for everyone?

        let sys_read: u64 = 0;
        let sys_write : u64 = 1;
        let sys_open: u64 = 2;
        let sys_close: u64 = 3;

        // return value
        let mut ret_value: u64;

        unsafe{
                // Open
                asm!(   "syscall",
                        in("rax") sys_open,
                        in("rdi") name_ptr,
                        in("rsi") flags,
                        in("rdx") mode,
                        lateout("rax") fd);


                if fd < 0 {
                        println!("Failed: fd < 0");
                } else {
                        // Write
                        asm!(   "syscall",
                                in("rax") sys_write,
                                in("rdi") fd,
                                in("rsi") message_ptr,
                                in("rdx") length,
                                lateout("rax") ret_value);

                        if ret_value != length as u64 {
                                println!("value from write was {}", ret_value);
                        } else {
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

                                // Read
                                asm!(   "syscall",
                                        in("rax") sys_read,
                                        in("rdi") fd,
                                        in("rsi") tmp_ptr,
                                        in("rdx") length);

                                // Close
                                asm!(   "syscall",
                                        in("rax") sys_close,
                                        in("rdi") fd);

                                println!("{}", message);
                        } // else               

                } // else
        } // unsafe
} // main


