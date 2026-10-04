// original: 0x0061a570 struct_combine_add (proposed)

/// Add two structs lane by lane into a third.
///
/// `dst` (ECX in the original), `src_a` (EDX) and `src_b` (first stack word)
/// point at the shared `0x118`-byte struct the three sibling routines pass
/// around. Every computed lane of `dst` is `the lane-wise sum of the matching `src_a` and `src_b` lanes`; the seven `w` lanes
/// (`0x0c`, `0x1c`, `0x2c`, `0x5c`, `0xac`, `0xcc`, `0xfc`) copy one word of
/// uninitialized stack (`[esp+0x1c]` in the original frame, below its entry
/// ESP, never written by the function), which the checker pins to
/// `STACK_FILL` through a defined stack fill. Lanes `0x9c`, `0xb4..0xbc`
/// and `0xe4..0xec` are never written and keep their old contents.
///
/// Calling convention: the original takes `dst` in ECX, `src_a` in EDX and
/// `src_b` as one stack word with caller cleanup (plain `ret`), which no
/// Rust convention expresses; the rewrite takes the same three words as
/// plain caller-cleanup arguments, in the order the contract delivers them
/// (`src_b` first, since the original reads it from its first stack word).
/// Lane order and operand order are the original's; arithmetic goes through
/// `black_box` helpers so the compiler cannot commute operands. Returns
/// `dst`. No calls, no globals.
lf_checker_rt::export!(cdecl, rw_0061a570(src_b: u32, dst: u32, src_a: u32) -> u32 {
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let (ecx, edx, esi) = (dst, src_a, src_b);

        let t1 = rdf(edx + 0x8);
        let t2 = rdf(edx + 0x0);
        let t3 = rdf(edx + 0x4);
        let t4 = add(t1, rdf(esi + 0x8));
        let t5 = add(t2, rdf(esi + 0x0));
        let t6 = add(t3, rdf(esi + 0x4));
        wrf(ecx + 0x8, t4);
        wrf(ecx + 0x0, t5);
        wrf(ecx + 0x4, t6);
        let t7 = STACK_FILL;
        wrf(ecx + 0xc, t7);
        let t8 = rdf(edx + 0x18);
        let t9 = add(t8, rdf(esi + 0x18));
        let t10 = rdf(edx + 0x10);
        let t11 = add(t10, rdf(esi + 0x10));
        let t12 = rdf(edx + 0x14);
        let t13 = add(t12, rdf(esi + 0x14));
        wrf(ecx + 0x18, t9);
        let t14 = STACK_FILL;
        wrf(ecx + 0x1c, t14);
        wrf(ecx + 0x10, t11);
        wrf(ecx + 0x14, t13);
        let t15 = rdf(edx + 0x34);
        let t16 = add(t15, rdf(esi + 0x34));
        let t17 = rdf(edx + 0x38);
        let t18 = add(t17, rdf(esi + 0x38));
        let t19 = rdf(edx + 0x3c);
        let t20 = add(t19, rdf(esi + 0x3c));
        let t21 = rdf(esi + 0x30);
        let t22 = add(t21, rdf(edx + 0x30));
        wrf(ecx + 0x34, t16);
        wrf(ecx + 0x38, t18);
        wrf(ecx + 0x3c, t20);
        wrf(ecx + 0x30, t22);
        let t23 = rdf(edx + 0x44);
        let t24 = add(t23, rdf(esi + 0x44));
        wrf(ecx + 0x44, t24);
        let t25 = rdf(edx + 0x40);
        let t26 = add(t25, rdf(esi + 0x40));
        wrf(ecx + 0x40, t26);
        let t27 = rdf(edx + 0x28);
        let t28 = add(t27, rdf(esi + 0x28));
        let t29 = rdf(edx + 0x20);
        let t30 = rdf(edx + 0x24);
        let t31 = add(t29, rdf(esi + 0x20));
        let t32 = add(t30, rdf(esi + 0x24));
        wrf(ecx + 0x28, t28);
        let t33 = STACK_FILL;
        wrf(ecx + 0x2c, t33);
        wrf(ecx + 0x20, t31);
        wrf(ecx + 0x24, t32);
        let t34 = rdf(edx + 0x48);
        let t35 = add(t34, rdf(esi + 0x48));
        let t36 = rdf(edx + 0x4c);
        let t37 = add(t36, rdf(esi + 0x4c));
        wr32(ecx + 0x48, t35.to_bits());
        wr32(ecx + 0x4c, t37.to_bits());
        let t38 = rdf(edx + 0x58);
        let t39 = add(t38, rdf(esi + 0x58));
        let t40 = rdf(esi + 0x50);
        let t41 = rdf(edx + 0x54);
        let t42 = add(t40, rdf(edx + 0x50));
        let t43 = add(t41, rdf(esi + 0x54));
        wrf(ecx + 0x58, t39);
        let t44 = STACK_FILL;
        wrf(ecx + 0x5c, t44);
        wrf(ecx + 0x50, t42);
        wrf(ecx + 0x54, t43);
        let t45 = rdf(edx + 0x60);
        let t46 = add(t45, rdf(esi + 0x60));
        wrf(ecx + 0x60, t46);
        let t47 = rdf(edx + 0x64);
        let t48 = add(t47, rdf(esi + 0x64));
        wrf(ecx + 0x64, t48);
        let t49 = rdf(edx + 0x68);
        let t50 = add(t49, rdf(esi + 0x68));
        wrf(ecx + 0x68, t50);
        let t51 = rdf(edx + 0x6c);
        let t52 = add(t51, rdf(esi + 0x6c));
        wrf(ecx + 0x6c, t52);
        let t53 = rdf(edx + 0x70);
        let t54 = add(t53, rdf(esi + 0x70));
        wrf(ecx + 0x70, t54);
        let t55 = rdf(edx + 0x74);
        let t56 = add(t55, rdf(esi + 0x74));
        wrf(ecx + 0x74, t56);
        let t57 = rdf(edx + 0x78);
        let t58 = add(t57, rdf(esi + 0x78));
        wrf(ecx + 0x78, t58);
        let t59 = rdf(edx + 0x7c);
        let t60 = add(t59, rdf(esi + 0x7c));
        wrf(ecx + 0x7c, t60);
        let t61 = rdf(edx + 0x80);
        let t62 = add(t61, rdf(esi + 0x80));
        wrf(ecx + 0x80, t62);
        let t63 = rdf(edx + 0x84);
        let t64 = add(t63, rdf(esi + 0x84));
        wrf(ecx + 0x84, t64);
        let t65 = rdf(edx + 0x88);
        let t66 = add(t65, rdf(esi + 0x88));
        wrf(ecx + 0x88, t66);
        let t67 = rdf(edx + 0xd4);
        let t68 = add(t67, rdf(esi + 0xd4));
        wrf(ecx + 0xd4, t68);
        let t69 = rdf(edx + 0x8c);
        let t70 = add(t69, rdf(esi + 0x8c));
        wrf(ecx + 0x8c, t70);
        let t71 = rdf(edx + 0x90);
        let t72 = add(t71, rdf(esi + 0x90));
        wrf(ecx + 0x90, t72);
        let t73 = rdf(edx + 0x94);
        let t74 = add(t73, rdf(esi + 0x94));
        wrf(ecx + 0x94, t74);
        let t75 = rdf(edx + 0x98);
        let t76 = add(t75, rdf(esi + 0x98));
        wrf(ecx + 0x98, t76);
        let t77 = rdf(edx + 0xa8);
        let t78 = add(t77, rdf(esi + 0xa8));
        let t79 = rdf(edx + 0xa0);
        let t80 = rdf(edx + 0xa4);
        let t81 = add(t79, rdf(esi + 0xa0));
        let t82 = add(t80, rdf(esi + 0xa4));
        wrf(ecx + 0xa8, t78);
        let t83 = STACK_FILL;
        wrf(ecx + 0xac, t83);
        wrf(ecx + 0xa0, t81);
        wrf(ecx + 0xa4, t82);
        let t84 = rdf(edx + 0xb0);
        let t85 = add(t84, rdf(esi + 0xb0));
        wrf(ecx + 0xb0, t85);
        let t86 = rdf(edx + 0xc8);
        let t87 = add(t86, rdf(esi + 0xc8));
        let t88 = rdf(edx + 0xc0);
        let t89 = rdf(edx + 0xc4);
        let t90 = add(t88, rdf(esi + 0xc0));
        let t91 = add(t89, rdf(esi + 0xc4));
        wrf(ecx + 0xc8, t87);
        let t92 = STACK_FILL;
        wrf(ecx + 0xcc, t92);
        wrf(ecx + 0xc0, t90);
        wrf(ecx + 0xc4, t91);
        let t93 = rdf(edx + 0xd0);
        let t94 = add(t93, rdf(esi + 0xd0));
        wrf(ecx + 0xd0, t94);
        let t95 = rdf(edx + 0x108);
        let t96 = add(t95, rdf(esi + 0x108));
        wrf(ecx + 0x108, t96);
        let t97 = rdf(edx + 0x10c);
        let t98 = add(t97, rdf(esi + 0x10c));
        wrf(ecx + 0x10c, t98);
        let t99 = rdf(edx + 0x110);
        let t100 = add(t99, rdf(esi + 0x110));
        wrf(ecx + 0x110, t100);
        let t101 = rdf(edx + 0x114);
        let t102 = add(t101, rdf(esi + 0x114));
        wrf(ecx + 0x114, t102);
        let t103 = rdf(edx + 0xd8);
        let t104 = add(t103, rdf(esi + 0xd8));
        wrf(ecx + 0xd8, t104);
        let t105 = rdf(edx + 0xdc);
        let t106 = add(t105, rdf(esi + 0xdc));
        wrf(ecx + 0xdc, t106);
        let t107 = rdf(edx + 0xe0);
        let t108 = add(t107, rdf(esi + 0xe0));
        wrf(ecx + 0xe0, t108);
        let t109 = rdf(edx + 0xf8);
        let t110 = add(t109, rdf(esi + 0xf8));
        let t111 = rdf(esi + 0xf0);
        let t112 = rdf(edx + 0xf4);
        let t113 = add(t112, rdf(esi + 0xf4));
        let t114 = add(t111, rdf(edx + 0xf0));
        wrf(ecx + 0xf8, t110);
        let t115 = STACK_FILL;
        wrf(ecx + 0xfc, t115);
        wrf(ecx + 0xf4, t113);
        wrf(ecx + 0xf0, t114);
        let t116 = rdf(edx + 0x100);
        let t117 = add(t116, rdf(esi + 0x100));
        wrf(ecx + 0x100, t117);
        let t118 = rdf(edx + 0x104);
        let t119 = add(t118, rdf(esi + 0x104));
        wrf(ecx + 0x104, t119);
        dst
    }
});
