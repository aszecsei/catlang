	.text
	.def	 @feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"simd.c"
	.def	 sumint;
	.scl	2;
	.type	32;
	.endef
	.globl	sumint                  # -- Begin function sumint
	.p2align	4, 0x90
sumint:                                 # @sumint
# %bb.0:
	testl	%edx, %edx
	jle	.LBB0_1
# %bb.2:
	movl	%edx, %r8d
	cmpl	$31, %edx
	ja	.LBB0_4
# %bb.3:
	xorl	%edx, %edx
	xorl	%eax, %eax
	jmp	.LBB0_11
.LBB0_1:
	xorl	%eax, %eax
	retq
.LBB0_4:
	movl	%r8d, %edx
	andl	$-32, %edx
	leaq	-32(%rdx), %r10
	movq	%r10, %rax
	shrq	$5, %rax
	addq	$1, %rax
	movl	%eax, %r9d
	andl	$1, %r9d
	testq	%r10, %r10
	je	.LBB0_5
# %bb.6:
	movq	%r9, %r10
	subq	%rax, %r10
	vpxor	%xmm0, %xmm0, %xmm0
	xorl	%eax, %eax
	vpxor	%xmm1, %xmm1, %xmm1
	vpxor	%xmm2, %xmm2, %xmm2
	vpxor	%xmm3, %xmm3, %xmm3
	.p2align	4, 0x90
.LBB0_7:                                # =>This Inner Loop Header: Depth=1
	vpaddd	(%rcx,%rax,4), %ymm0, %ymm0
	vpaddd	32(%rcx,%rax,4), %ymm1, %ymm1
	vpaddd	64(%rcx,%rax,4), %ymm2, %ymm2
	vpaddd	96(%rcx,%rax,4), %ymm3, %ymm3
	vpaddd	128(%rcx,%rax,4), %ymm0, %ymm0
	vpaddd	160(%rcx,%rax,4), %ymm1, %ymm1
	vpaddd	192(%rcx,%rax,4), %ymm2, %ymm2
	vpaddd	224(%rcx,%rax,4), %ymm3, %ymm3
	addq	$64, %rax
	addq	$2, %r10
	jne	.LBB0_7
# %bb.8:
	testq	%r9, %r9
	je	.LBB0_10
.LBB0_9:
	vpaddd	96(%rcx,%rax,4), %ymm3, %ymm3
	vpaddd	64(%rcx,%rax,4), %ymm2, %ymm2
	vpaddd	32(%rcx,%rax,4), %ymm1, %ymm1
	vpaddd	(%rcx,%rax,4), %ymm0, %ymm0
.LBB0_10:
	vpaddd	%ymm3, %ymm1, %ymm1
	vpaddd	%ymm2, %ymm0, %ymm0
	vpaddd	%ymm1, %ymm0, %ymm0
	vextracti128	$1, %ymm0, %xmm1
	vpaddd	%xmm1, %xmm0, %xmm0
	vpshufd	$78, %xmm0, %xmm1       # xmm1 = xmm0[2,3,0,1]
	vpaddd	%xmm1, %xmm0, %xmm0
	vpshufd	$229, %xmm0, %xmm1      # xmm1 = xmm0[1,1,2,3]
	vpaddd	%xmm1, %xmm0, %xmm0
	vmovd	%xmm0, %eax
	cmpq	%r8, %rdx
	je	.LBB0_12
	.p2align	4, 0x90
.LBB0_11:                               # =>This Inner Loop Header: Depth=1
	addl	(%rcx,%rdx,4), %eax
	addq	$1, %rdx
	cmpq	%rdx, %r8
	jne	.LBB0_11
.LBB0_12:
	vzeroupper
	retq
.LBB0_5:
	vpxor	%xmm0, %xmm0, %xmm0
	xorl	%eax, %eax
	vpxor	%xmm1, %xmm1, %xmm1
	vpxor	%xmm2, %xmm2, %xmm2
	vpxor	%xmm3, %xmm3, %xmm3
	testq	%r9, %r9
	jne	.LBB0_9
	jmp	.LBB0_10
                                        # -- End function
	.def	 main;
	.scl	2;
	.type	32;
	.endef
	.globl	__ymm@000000a8000000a20000009c00000096000000900000008a000000840000007e # -- Begin function main
	.section	.rdata,"dr",discard,__ymm@000000a8000000a20000009c00000096000000900000008a000000840000007e
	.p2align	5
__ymm@000000a8000000a20000009c00000096000000900000008a000000840000007e:
	.long	126                     # 0x7e
	.long	132                     # 0x84
	.long	138                     # 0x8a
	.long	144                     # 0x90
	.long	150                     # 0x96
	.long	156                     # 0x9c
	.long	162                     # 0xa2
	.long	168                     # 0xa8
	.text
	.globl	main
	.p2align	4, 0x90
main:                                   # @main
.seh_proc main
# %bb.0:
	subq	$200, %rsp
	.seh_stackalloc 200
	.seh_endprologue
	vmovdqa	__ymm@000000a8000000a20000009c00000096000000900000008a000000840000007e+16(%rip), %xmm0
	vpaddd	__ymm@000000a8000000a20000009c00000096000000900000008a000000840000007e(%rip), %xmm0, %xmm0
	vpshufd	$78, %xmm0, %xmm1       # xmm1 = xmm0[2,3,0,1]
	vpaddd	%xmm1, %xmm0, %xmm0
	vpshufd	$229, %xmm0, %xmm1      # xmm1 = xmm0[1,1,2,3]
	vpaddd	%xmm1, %xmm0, %xmm0
	vmovd	%xmm0, %eax
	addl	$99, %eax
	addq	$200, %rsp
	retq
	.seh_handlerdata
	.text
	.seh_endproc
                                        # -- End function
	.addrsig
