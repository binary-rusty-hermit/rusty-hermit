        .global _start

        .text
_start:
        # open(filename, flags, mode)	# open a file
        mov     SYS_OPEN, %rax
        mov     $filename, %rdi
        mov     flags, %rsi
        mov     mode, %rdx
        syscall                         # invoke operating system to do the open

        cmp     $0, %rax               	# if file descriptor is less than or equal to zero
        jle     exit                    # jump to exit and terminate
       	mov     %rax, fd


	# read(fd, inputString, 12)	# read from the file  into inputString
        mov     SYS_READ, %rax
        mov     fd, %rdi
        mov     $inputString, %rsi
        mov     $12, %rdx
        syscall				# invoke operating system to do the read

	cmp	$12, %rax		# if 12 bytes were not read something went wrong,
	jne	close			# close the file and terminate the program


	# write(fd, inputString, 12)	# now we print the value to standard output
        mov     SYS_WRITE, %rax
        mov     STDOUT, %rdi
        mov     $inputString, %rsi
        mov     $12, %rdx
        syscall                         # invoke operating system to do the write

	cmp     $12, %rax
        jne     close


	# lseek(fd, newOffset, whence)	# move the file offset to the beginning of the file
	mov     SYS_LSEEK, %rax
        mov     fd, %rdi
        mov     $-5, %rsi
        mov     $1, %rdx		# We want to set the file offset the current offset + newOffset
        syscall

	cmp 	$0, %rax
	jl	close

	# write(fd, message, 7)        # write message to the file at the new file offset
        mov     SYS_WRITE, %rax
        mov     fd, %rdi
        mov     $message, %rsi
        mov     $7, %rdx
        syscall

close:  # close(fd)			# close the file
        mov     SYS_CLOSE, %rax
        mov     fd, %rdi
	syscall

exit: 	# exit(0)			# terminate the program
        mov     SYS_EXIT, %rax
        xor     %rdi, %rdi              # we want return code 0
        syscall

.section        .data
message:
        .ascii  "world!\n"
.skip 8
filename:
        .ascii  "file.txt\0"

.skip 8
flags:  .word 0x0102
.skip 8
mode:   .word 0x309
.skip 8

STDOUT: .word   1
.skip 8
SYS_READ:       .word   0
.skip 8
SYS_WRITE:      .word   1
.skip 8
SYS_OPEN:       .word   2
.skip 8
SYS_CLOSE:      .word   3
.skip 8
SYS_LSEEK:       .word   8
.skip 8
SYS_EXIT:       .word   60
.skip 8

.section 	.bss
.lcomm fd, 8
.lcomm inputString, 12
