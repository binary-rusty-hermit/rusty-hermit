#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let message = "Message in Std out\n";
        let length = message.len();

        let tmp = "     ";
        let tmp_ptr = tmp.as_ptr();

        let fd : u64 = 1;
        let pointer = message.as_ptr();
        let sys_write : u64 = 1;
        //let sys_exit : u64 = 60;

        // return value
        let mut ret_value: u64;

        unsafe{
                asm!(   "syscall",
                        in("rax") sys_write,
                        in("rdi") fd,
                        in("rsi") pointer,
                        in("rdx") length,
                        lateout("rax") ret_value
                        );

                let message = "Message in Std err\n";
                let length = message.len();
                let pointer = message.as_ptr();
                let fd : u64 = 2;

                asm!(   "syscall",

                        in("rax") sys_write,
                        in("rdi") fd,
                        in("rsi") pointer,
                        in("rdx") length

                        );

                let sys_read: u64 = 0;
                let fd: u64 = 1;
                let length: u64 = 5;

                asm!(   "syscall",
                        in("rax") sys_read,
                        in("rdi") fd,
                        in("rsi") tmp_ptr,
                        in("rdx") length,
                        lateout("rax") ret_value
                        );

                println!("{}", tmp);
                println!("{}", ret_value);

                let sys_exit = 60;
                let value = 1;
                asm!("syscall",
                        in("rax") sys_exit,
                        in("rdi") value);

        }
        println!("DONE");
}

/* To test values in State
asm!(   "          
                        mov rax, 1
                        mov rdx, 2
                        mov rbx, 3
                        mov rbp, 4
                        mov rsi, 5
                        mov rdi, 6
                        mov r8, 7
                        mov r9, 8
                        mov r10, 9
                        mov r12, 10
                        mov r13, 11
                        mov r14, 12
                        mov r15, 13
                        syscall"
                );
*/

