        .global _start

        .text
_start:
        # mmap(addr, len, prot, flags, fd, offset)	#  Request memory for read and write of length len
        mov     SYS_MMAP, %rax
        mov     $0, %rdi
        mov     len, %rsi		# amount of memory we want
        mov     prot, %rdx		# read & write
	mov	flags, %r10		# private & map_anonymous
	mov	$-1, %r8		# fd (not needed)
	mov	$0, %r9			# offset (not needed)
        syscall                         # invoke operating system to do the mmap

        cmp     $0, %rax               	# if the address of the block of memory allocated is zero then it failed
        je     exit                    # jump to exit and terminate
       	mov     %rax, addr



	# write(STDOUT, message, 9)    # now we print the value to standard output
        mov     SYS_WRITE, %rax
        mov     STDOUT, %rdi
        mov     $message, %rsi
        mov     $9, %rdx
        syscall                         # invoke operating system to do the write



	# munmap(addr, len)      #  Request memory allocated to be freed
        mov     SYS_MUNMAP, %rax
        mov     addr, %rdi		# address of the block of memory that was allocated
        mov     len, %rsi
	syscall

        cmp     $0, %rax               # if the address of the block of memory allocated is zero then it failed
        je     exit                    # jump to exit and terminate
        mov     %rax, addr


exit: 	# exit(0)			# terminate the program
        mov     SYS_EXIT, %rax
        xor     %rdi, %rdi              # we want return code 0
        syscall

.section        .data
message:
        .ascii  "Success!\n"
.skip 8
len:	.word 1024
.skip 8
prot:	.word 0x3
.skip 8
flags:  .word 0x022		# private & map_anonymous
.skip 8
mode:   .word 0x309
.skip 8

STDOUT: .word   1
.skip 8
SYS_READ:       .word   0
.skip 8
SYS_WRITE:      .word   1
.skip 8
SYS_MMAP:       .word   9
.skip 8
SYS_MUNMAP:      .word   11
.skip 8
SYS_MPROTECT:    .word   10
.skip 8
SYS_EXIT:       .word   60
.skip 8

.section 	.bss
.lcomm addr, 8
.lcomm inputString, 12
