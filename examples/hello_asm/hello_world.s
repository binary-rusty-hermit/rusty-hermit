        .global _start

        .text
_start:
	# write(STDOUT, message, 14)	# now we print the value to standard output
        mov     SYS_WRITE, %rax
        mov     STDOUT, %rdi
        mov     $message, %rsi
        mov     $14, %rdx
        syscall                         # invoke operating system to do the write

exit: 	# exit(0)			# terminate the program
        mov     SYS_EXIT, %rax
        xor     %rdi, %rdi              # we want return code 0
        syscall

.section 	.data
message:
        .ascii  "Hello, world!\n"
.skip 8

STDOUT: .word   1
.skip 8
SYS_WRITE:      .word   1
.skip 8
SYS_EXIT:      	.word   60
.skip 8

