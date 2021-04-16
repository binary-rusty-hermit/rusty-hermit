        .global _start

        .text
_start:

	# readlink(fd, buf, 12)	# perform readlink on the symbolic link
        mov     SYS_READLINK, %rax
        mov     $link_path, %rdi
        mov     $buf, %rsi
        mov     buf_len, %rdx
        syscall				# invoke operating system to do the readlink

	cmp	$0, %rax		# if 0 bytes were read from the symbolic link we have failed,
	je	exit			# close the file and terminate the program

	add	$1, %rax		# add a new line
	mov	%rax, readlink_len

	# write(STDOUT, buf, readlink_len)	# if we did not fail, print to standard output what readlink read
        mov     SYS_WRITE, %rax
        mov     STDOUT, %rdi
        mov     $buf, %rsi
        mov     readlink_len, %rdx
        syscall                         # invoke operating system to do the write

exit: 	# exit(0)			# terminate the program
        mov     SYS_EXIT, %rax
        xor     %rdi, %rdi              # we want return code 0
        syscall

.section        .data
link_path:
        .ascii  "sym_link\0"
.skip 8
buf_len:
	.word	20
.skip 8

STDOUT: .word   1
.skip 8
SYS_READLINK:   .word   89
.skip 8
SYS_WRITE:      .word   1
.skip 8
SYS_EXIT:       .word   60
.skip 8

.section 	.bss
.lcomm len, 8
.lcomm buf, 20
.lcomm readlink_len, 8
