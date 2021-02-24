#![feature(asm)]
#![allow(unused_imports)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
        let sys_arch_prctl: u64 = 158;
        let arch_set_fs: u64 = 0x1002;
        let arch_get_fs: u64 = 0x1003;

        // return value
        let mut ret_value: u64;
        let addr: u64 = 158;
        let addr_ptr = &addr;
        unsafe{
                // Get fs
                asm!(   "syscall",
                        in("rax") sys_arch_prctl,
                        in("rdi") arch_get_fs,
                        in("rsi") addr_ptr,
                        lateout("rax") ret_value
                        );

                println!("Return value was {}\nFS register was {}", ret_value, addr);

                let addr = addr + 4;
                let addr_ptr = &addr;

                // Set fs
                asm!(   "syscall",
                        in("rax") sys_arch_prctl,
                        in("rdi") arch_set_fs,
                        in("rsi") addr,
                        lateout("rax") ret_value
                        );

                 println!("Return value was {}", ret_value);

                // Get fs
                asm!(   "syscall",
                        in("rax") sys_arch_prctl,
                        in("rdi") arch_get_fs,
                        in("rsi") addr_ptr,
                        lateout("rax") ret_value
                        );

                println!("Return value was {}\nFS register was {}", ret_value, addr);

        }

}
