// original: 0x0061ac30 struct_combine_scale (proposed)

/// Scale one struct by the leading lane of another.
///
/// `dst` (ECX in the original), `src_a` (EDX) and `src_b` (first stack word)
/// point at the shared `0x118`-byte struct the three sibling routines pass
/// around. Every computed lane of `dst` is `the `src_a` lane times the leading (`+0x00`) lane of `src_b``; the seven `w` lanes
/// (`0x0c`, `0x1c`, `0x2c`, `0x5c`, `0xac`, `0xcc`, `0xfc`) copy one word of
/// uninitialized stack (`[esp+0x1c]` in the original frame, below its entry
/// ESP, never written by the function), which the checker pins to
/// `STACK_FILL` through a defined stack fill. Lanes `0x9c`, `0xb4..0xbc`
/// and `0xe4..0xec` are never written and keep their old contents.
///
/// Calling convention: the original takes `dst` in ECX, `src_a` in EDX and
/// `src_b` as one stack word with caller cleanup (the caller pops the
/// words), which no
/// Rust convention expresses; the rewrite takes the same three words as
/// plain caller-cleanup arguments, in the order the contract delivers them
/// (`src_b` first, since the original reads it from its first stack word).
/// Lane order and operand order are the original's; arithmetic goes through
/// `black_box` helpers so the compiler cannot commute operands. Returns
/// `dst`. No calls, no globals.
lf_checker_rt::export!(cdecl, rw_0061ac30(src_b: u32, dst: u32, src_a: u32) -> u32 {
    unsafe {
        const STACK_FILL: f32 = 0.0;
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let (ecx, edx, esi) = (dst, src_a, src_b);

        let t1 = rdf(edx + 0x8);
        let t2 = rdf(edx + 0x0);
        let t3 = rdf(edx + 0x4);
        let t4 = rdf(esi + 0x0);
        let t5 = mul(t1, t4);
        let t6 = mul(t2, t4);
        wrf(ecx + 0x8, t5);
        let t7 = STACK_FILL;
        wrf(ecx + 0xc, t7);
        wrf(ecx + 0x0, t6);
        let t8 = mul(t3, t4);
        wrf(ecx + 0x4, t8);
        let t9 = rdf(esi + 0x0);
        let t10 = rdf(edx + 0x18);
        let t11 = rdf(edx + 0x14);
        let t12 = mul(t10, t9);
        let t13 = mul(t11, t9);
        let t14 = mul(t9, rdf(edx + 0x10));
        wrf(ecx + 0x18, t12);
        wrf(ecx + 0x14, t13);
        wrf(ecx + 0x10, t14);
        let t15 = STACK_FILL;
        wrf(ecx + 0x1c, t15);
        let t16 = rdf(esi + 0x0);
        let t17 = rdf(edx + 0x34);
        let t18 = rdf(edx + 0x38);
        let t19 = rdf(edx + 0x3c);
        let t20 = mul(t19, t16);
        let t21 = mul(t17, t16);
        let t22 = mul(t16, rdf(edx + 0x30));
        wrf(ecx + 0x3c, t20);
        wrf(ecx + 0x34, t21);
        wrf(ecx + 0x30, t22);
        let t23 = mul(t18, t16);
        wrf(ecx + 0x38, t23);
        let t24 = rdf(edx + 0x44);
        let t25 = mul(t24, rdf(esi + 0x0));
        wrf(ecx + 0x44, t25);
        let t26 = rdf(esi + 0x0);
        let t27 = rdf(edx + 0x28);
        let t28 = rdf(edx + 0x20);
        let t29 = rdf(edx + 0x24);
        let t30 = mul(t27, t26);
        let t31 = mul(t28, t26);
        wrf(ecx + 0x28, t30);
        let t32 = STACK_FILL;
        wrf(ecx + 0x2c, t32);
        wrf(ecx + 0x20, t31);
        let t33 = mul(t29, t26);
        wrf(ecx + 0x24, t33);
        let t34 = rdf(edx + 0x48);
        let t35 = mul(t34, rdf(esi + 0x0));
        let t36 = rdf(edx + 0x4c);
        let t37 = mul(t36, rdf(esi + 0x0));
        wr32(ecx + 0x48, t35.to_bits());
        wr32(ecx + 0x4c, t37.to_bits());
        let t38 = rdf(edx + 0x40);
        let t39 = mul(t38, rdf(esi + 0x0));
        wrf(ecx + 0x40, t39);
        let t40 = rdf(esi + 0x0);
        let t41 = rdf(edx + 0x58);
        let t42 = rdf(edx + 0x50);
        let t43 = rdf(edx + 0x54);
        let t44 = mul(t41, t40);
        let t45 = mul(t42, t40);
        let t46 = mul(t43, t40);
        wrf(ecx + 0x58, t44);
        let t47 = STACK_FILL;
        wrf(ecx + 0x50, t45);
        wrf(ecx + 0x54, t46);
        wrf(ecx + 0x5c, t47);
        let t48 = rdf(edx + 0x60);
        let t49 = mul(t48, rdf(esi + 0x0));
        wrf(ecx + 0x60, t49);
        let t50 = rdf(edx + 0x64);
        let t51 = mul(t50, rdf(esi + 0x0));
        wrf(ecx + 0x64, t51);
        let t52 = rdf(edx + 0x68);
        let t53 = mul(t52, rdf(esi + 0x0));
        wrf(ecx + 0x68, t53);
        let t54 = rdf(edx + 0x6c);
        let t55 = mul(t54, rdf(esi + 0x0));
        wrf(ecx + 0x6c, t55);
        let t56 = rdf(edx + 0x70);
        let t57 = mul(t56, rdf(esi + 0x0));
        wrf(ecx + 0x70, t57);
        let t58 = rdf(edx + 0x74);
        let t59 = mul(t58, rdf(esi + 0x0));
        wrf(ecx + 0x74, t59);
        let t60 = rdf(edx + 0x78);
        let t61 = mul(t60, rdf(esi + 0x0));
        wrf(ecx + 0x78, t61);
        let t62 = rdf(edx + 0x7c);
        let t63 = mul(t62, rdf(esi + 0x0));
        wrf(ecx + 0x7c, t63);
        let t64 = rdf(edx + 0x80);
        let t65 = mul(t64, rdf(esi + 0x0));
        wrf(ecx + 0x80, t65);
        let t66 = rdf(edx + 0x84);
        let t67 = mul(t66, rdf(esi + 0x0));
        wrf(ecx + 0x84, t67);
        let t68 = rdf(edx + 0x88);
        let t69 = mul(t68, rdf(esi + 0x0));
        wrf(ecx + 0x88, t69);
        let t70 = rdf(edx + 0x8c);
        let t71 = mul(t70, rdf(esi + 0x0));
        wrf(ecx + 0x8c, t71);
        let t72 = rdf(edx + 0x90);
        let t73 = mul(t72, rdf(esi + 0x0));
        wrf(ecx + 0x90, t73);
        let t74 = rdf(edx + 0x94);
        let t75 = mul(t74, rdf(esi + 0x0));
        wrf(ecx + 0x94, t75);
        let t76 = rdf(edx + 0x98);
        let t77 = mul(t76, rdf(esi + 0x0));
        wrf(ecx + 0x98, t77);
        let t78 = rdf(esi + 0x0);
        let t79 = rdf(edx + 0xa8);
        let t80 = rdf(edx + 0xa0);
        let t81 = rdf(edx + 0xa4);
        let t82 = mul(t79, t78);
        let t83 = mul(t80, t78);
        wrf(ecx + 0xa8, t82);
        let t84 = STACK_FILL;
        wrf(ecx + 0xac, t84);
        wrf(ecx + 0xa0, t83);
        let t85 = mul(t81, t78);
        wrf(ecx + 0xa4, t85);
        let t86 = rdf(edx + 0xb0);
        let t87 = mul(t86, rdf(esi + 0x0));
        wrf(ecx + 0xb0, t87);
        let t88 = rdf(esi + 0x0);
        let t89 = rdf(edx + 0xc8);
        let t90 = rdf(edx + 0xc0);
        let t91 = rdf(edx + 0xc4);
        let t92 = mul(t89, t88);
        let t93 = mul(t90, t88);
        wrf(ecx + 0xc8, t92);
        let t94 = STACK_FILL;
        wrf(ecx + 0xc0, t93);
        wrf(ecx + 0xcc, t94);
        let t95 = mul(t91, t88);
        wrf(ecx + 0xc4, t95);
        let t96 = rdf(edx + 0xd0);
        let t97 = mul(t96, rdf(esi + 0x0));
        wrf(ecx + 0xd0, t97);
        let t98 = rdf(edx + 0xd4);
        let t99 = mul(t98, rdf(esi + 0x0));
        wrf(ecx + 0xd4, t99);
        let t100 = rdf(edx + 0x108);
        let t101 = mul(t100, rdf(esi + 0x0));
        wrf(ecx + 0x108, t101);
        let t102 = rdf(edx + 0x10c);
        let t103 = mul(t102, rdf(esi + 0x0));
        wrf(ecx + 0x10c, t103);
        let t104 = rdf(edx + 0x110);
        let t105 = mul(t104, rdf(esi + 0x0));
        wrf(ecx + 0x110, t105);
        let t106 = rdf(edx + 0x114);
        let t107 = mul(t106, rdf(esi + 0x0));
        wrf(ecx + 0x114, t107);
        let t108 = rdf(edx + 0xd8);
        let t109 = mul(t108, rdf(esi + 0x0));
        wrf(ecx + 0xd8, t109);
        let t110 = rdf(edx + 0xdc);
        let t111 = mul(t110, rdf(esi + 0x0));
        wrf(ecx + 0xdc, t111);
        let t112 = rdf(edx + 0xe0);
        let t113 = mul(t112, rdf(esi + 0x0));
        wrf(ecx + 0xe0, t113);
        let t114 = rdf(esi + 0x0);
        let t115 = rdf(edx + 0xf8);
        let t116 = rdf(edx + 0xf0);
        let t117 = rdf(edx + 0xf4);
        let t118 = mul(t115, t114);
        let t119 = mul(t116, t114);
        wrf(ecx + 0xf8, t118);
        let t120 = STACK_FILL;
        let t121 = mul(t117, t114);
        wrf(ecx + 0xf0, t119);
        wrf(ecx + 0xfc, t120);
        wrf(ecx + 0xf4, t121);
        let t122 = rdf(edx + 0x100);
        let t123 = mul(t122, rdf(esi + 0x0));
        wrf(ecx + 0x100, t123);
        let t124 = rdf(edx + 0x104);
        let t125 = mul(t124, rdf(esi + 0x0));
        wrf(ecx + 0x104, t125);
        dst
    }
});
