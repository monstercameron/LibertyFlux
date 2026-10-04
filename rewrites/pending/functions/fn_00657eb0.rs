// original: 0x00657eb0 frag_cache_refresh_derived_rows
/// Refresh the derived direction rows of a fragment-cache entry.
///
/// Normalizes (or zeroes, when the squared length is exactly zero) three
/// input rows, then rebuilds the derived rows from them and the entry
/// parameters. Three float square-root calls (one per row) with a
/// normalize-or-zero guard each; everything else is straight-line SSE
/// arithmetic over object fields. Original: thiscall/0, no return value.
///
/// Ordering note: several float ops select between two NaN inputs, where SSE
/// propagates the FIRST operand, so operand order is observable bit-for-bit.
/// Plain Rust `a + b`/`a * b` is miscompiled here (the backend commutes
/// operands when folding spills, and `black_box` does not stop it), so every
/// arithmetic op runs through the call-barrier helpers below, which emit one
/// ordered SSE op on stack-passed operands and return bits.
#[inline(always)]
fn rf(obj: *mut u8, off: usize) -> f32 {
    // SAFETY: the checker guarantees `obj` points to a live heap object; all
    // offsets used are inside the declared segment.
    unsafe { ((obj as *const u8).add(off) as *const f32).read_unaligned() }
}
#[inline(always)]
fn wf(obj: *mut u8, off: usize, v: f32) {
    // SAFETY: same contract as `rf`.
    unsafe { ((obj as *mut u8).add(off) as *mut f32).write_unaligned(v) }
}
/// Bit-exact float xor (the original's `xorps` on float lanes).
#[inline(always)]
fn fxor(a: f32, b: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ b.to_bits())
}
// Ordered float arithmetic: LLVM's backend commutes the operands of plain
// `a + b`/`a * b` (observed: systematic NaN-sign diffs; SSE propagates the
// FIRST NaN operand, so order is observable), and `black_box` on the operands
// does NOT stop it (proven: one swapped add survived it). Each op therefore
// runs behind a call boundary with stack-passed operands, where no commute
// is profitable, and returns bits so no x87 round-trip is involved.
#[inline(never)]
fn fadd_c(a: f32, b: f32) -> u32 {
    (core::hint::black_box(a) + core::hint::black_box(b)).to_bits()
}
#[inline(never)]
fn fsub_c(a: f32, b: f32) -> u32 {
    (core::hint::black_box(a) - core::hint::black_box(b)).to_bits()
}
#[inline(never)]
fn fmul_c(a: f32, b: f32) -> u32 {
    (core::hint::black_box(a) * core::hint::black_box(b)).to_bits()
}
#[inline(never)]
fn fdiv_c(a: f32, b: f32) -> u32 {
    (core::hint::black_box(a) / core::hint::black_box(b)).to_bits()
}

const ZERO: f32 = 0.0;

// Refresh the derived direction rows of a fragment-cache entry: normalize
// (or zero) three input rows, then rebuild rows 0x210..0x288 from them and
// the entry parameters. Original: thiscall/0, no return value.
export!(thiscall, rw_00657eb0(obj: *mut u8) -> u32 {
    let g_one: f32 = unsafe { (global::<f32>(0x00FE88E8) as *const f32).read_unaligned() };
    let g_sign: f32 = unsafe { (global::<f32>(0x00FE8FA0) as *const f32).read_unaligned() };
    let t1: f32 = rf(obj, 0xE8); // 0x00657EC2
    let t2: f32 = rf(obj, 0x100); // 0x00657ECA
    let t3: f32 = rf(obj, 0x104); // 0x00657ED2
    let t4: f32 = rf(obj, 0x108); // 0x00657EDA
    let t5: f32 = rf(obj, 0xF0); // 0x00657EE8
    let t6: f32 = rf(obj, 0xF4); // 0x00657EF6
    let t7: f32 = rf(obj, 0xF8); // 0x00657F04
    let t8: f32 = fxor(t3, g_sign); // 0x00657F1A
    let t9: f32 = fxor(t2, g_sign); // 0x00657F1D
    let t10: f32 = fxor(t4, g_sign); // 0x00657F20
    let t11: f32 = f32::from_bits(fmul_c(t9, t9)); // 0x00657F29 mulss
    let t12: f32 = rf(obj, 0xE0); // 0x00657F2D
    let t13: f32 = rf(obj, 0xE4); // 0x00657F35
    let t14: f32 = f32::from_bits(fmul_c(t8, t8)); // 0x00657F3D mulss
    let t15: f32 = f32::from_bits(fadd_c(t14, t11)); // 0x00657F4D addss
    let t16: f32 = f32::from_bits(fmul_c(t10, t10)); // 0x00657F54 mulss
    let t17: f32 = f32::from_bits(fadd_c(t15, t16)); // 0x00657F64 addss
    let k1: f32 = if t17 == 0.0 { // diamond 0x00657F75
        ZERO
    } else {
    let sqrt1: f32 = callee_cdecl!(1, f32, t17.to_bits()); // 0x00657F82
    let t18: f32 = f32::from_bits(fdiv_c(g_one, sqrt1)); // 0x00657F96 divss
        t18
    };
    let t19: f32 = f32::from_bits(fmul_c(t9, k1)); // 0x00657FC3 mulss
    let t20: f32 = f32::from_bits(fmul_c(t8, k1)); // 0x00657FC7 mulss
    let t21: f32 = f32::from_bits(fmul_c(t10, k1)); // 0x00657FCB mulss
    let t22: f32 = f32::from_bits(fmul_c(t12, t12)); // 0x00657FD5 mulss
    let t23: f32 = f32::from_bits(fmul_c(t13, t13)); // 0x00657FD9 mulss
    let t24: f32 = f32::from_bits(fadd_c(t23, t22)); // 0x00657FE9 addss
    let t25: f32 = f32::from_bits(fmul_c(t1, t1)); // 0x00657FF0 mulss
    let t26: f32 = f32::from_bits(fadd_c(t24, t25)); // 0x00658000 addss
    let k2: f32 = if t26 == 0.0 { // diamond 0x0065800B
        ZERO
    } else {
    let sqrt2: f32 = callee_cdecl!(1, f32, t26.to_bits()); // 0x00658018
    let t27: f32 = f32::from_bits(fdiv_c(g_one, sqrt2)); // 0x0065802C divss
        t27
    };
    let t28: f32 = f32::from_bits(fmul_c(t1, k2)); // 0x00658059 mulss
    let t29: f32 = f32::from_bits(fmul_c(t12, k2)); // 0x0065805D mulss
    let t30: f32 = f32::from_bits(fmul_c(t13, k2)); // 0x00658061 mulss
    let t31: f32 = f32::from_bits(fmul_c(t5, t5)); // 0x00658077 mulss
    let t32: f32 = f32::from_bits(fmul_c(t6, t6)); // 0x0065807B mulss
    let t33: f32 = f32::from_bits(fadd_c(t32, t31)); // 0x0065808B addss
    let t34: f32 = f32::from_bits(fmul_c(t7, t7)); // 0x00658092 mulss
    let t35: f32 = f32::from_bits(fadd_c(t33, t34)); // 0x00658096 addss
    let k3: f32 = if t35 == 0.0 { // diamond 0x006580A1
        ZERO
    } else {
    let sqrt3: f32 = callee_cdecl!(1, f32, t35.to_bits()); // 0x006580A9
    let t36: f32 = f32::from_bits(fdiv_c(g_one, sqrt3)); // 0x006580BD divss
        t36
    };
    let t37: f32 = rf(obj, 0x130); // 0x006580DB
    let t38: f32 = f32::from_bits(fmul_c(t6, k3)); // 0x006580E9 mulss
    let t39: f32 = f32::from_bits(fmul_c(t7, k3)); // 0x006580ED mulss
    let t40: f32 = rf(obj, 0x138); // 0x006580F1
    let t41: f32 = f32::from_bits(fmul_c(t5, k3)); // 0x006580F9 mulss
    let t42: f32 = rf(obj, 0x134); // 0x006580FD
    let t43: f32 = f32::from_bits(fmul_c(t21, t37)); // 0x00658111 mulss
    let t44: f32 = f32::from_bits(fmul_c(t20, t37)); // 0x00658121 mulss
    let t45: f32 = f32::from_bits(fmul_c(t19, t37)); // 0x00658128 mulss
    let t46: f32 = f32::from_bits(fmul_c(t20, t42)); // 0x00658132 mulss
    let t47: f32 = f32::from_bits(fmul_c(t40, t42)); // 0x00658136 mulss
    let t48: f32 = rf(obj, 0x114); // 0x0065813A
    let t49: f32 = f32::from_bits(fadd_c(t46, t48)); // 0x0065813A addss
    let t50: f32 = f32::from_bits(fmul_c(t19, t42)); // 0x00658142 mulss
    let t51: f32 = rf(obj, 0x13C); // 0x0065814C
    let t52: f32 = f32::from_bits(fmul_c(t21, t42)); // 0x00658160 mulss
    let t53: f32 = rf(obj, 0x110); // 0x00658164
    let t54: f32 = f32::from_bits(fadd_c(t50, t53)); // 0x00658164 addss
    let t55: f32 = f32::from_bits(fmul_c(t51, t42)); // 0x0065816C mulss
    let t56: f32 = rf(obj, 0x118); // 0x00658170
    let t57: f32 = f32::from_bits(fadd_c(t52, t56)); // 0x00658170 addss
    let t58: f32 = f32::from_bits(fmul_c(t29, t47)); // 0x0065817E mulss
    let t59: f32 = f32::from_bits(fmul_c(t30, t47)); // 0x00658194 mulss
    let t60: f32 = f32::from_bits(fmul_c(t28, t47)); // 0x006581B9 mulss
    let t61: f32 = f32::from_bits(fadd_c(t45, t53)); // 0x006581BD addss
    let t62: f32 = f32::from_bits(fadd_c(t44, t48)); // 0x006581C5 addss
    let t63: f32 = fxor(t38, g_sign); // 0x006581DE
    let t64: f32 = fxor(t41, g_sign); // 0x006581E1
    let t65: f32 = fxor(t39, g_sign); // 0x006581F0
    let t66: f32 = f32::from_bits(fmul_c(t63, t55)); // 0x006581F9 mulss
    let t67: f32 = f32::from_bits(fadd_c(t43, t56)); // 0x006581FD addss
    let t68: f32 = f32::from_bits(fmul_c(t65, t55)); // 0x00658217 mulss
    let t69: f32 = f32::from_bits(fmul_c(t65, t55)); // 0x0065821B mulss
    let t70: f32 = f32::from_bits(fmul_c(t29, t47)); // 0x0065822E mulss
    wf(obj, 0x210, t61); // 0x00658232
    wf(obj, 0x214, t62); // 0x0065823A
    let t71: f32 = f32::from_bits(fmul_c(t30, t47)); // 0x0065824E mulss
    wf(obj, 0x218, t67); // 0x00658252
    let t72: f32 = f32::from_bits(fmul_c(t64, t55)); // 0x0065825A mulss
    let t73: f32 = f32::from_bits(fmul_c(t28, t47)); // 0x0065826A mulss
    let t74: f32 = f32::from_bits(fmul_c(t64, t55)); // 0x00658286 mulss
    let t75: f32 = f32::from_bits(fmul_c(t63, t55)); // 0x00658296 mulss
    wf(obj, 0x250, t54); // 0x006582A6
    wf(obj, 0x254, t49); // 0x006582AE
    wf(obj, 0x258, t57); // 0x006582BC
    wf(obj, 0x220, t61); // 0x006582C4
    wf(obj, 0x224, t62); // 0x006582CC
    wf(obj, 0x228, t67); // 0x006582D4
    wf(obj, 0x260, t54); // 0x006582DC
    wf(obj, 0x264, t49); // 0x006582E4
    wf(obj, 0x268, t57); // 0x006582EC
    wf(obj, 0x230, t61); // 0x006582F4
    wf(obj, 0x234, t62); // 0x006582FC
    wf(obj, 0x238, t67); // 0x00658304
    wf(obj, 0x270, t54); // 0x0065830C
    wf(obj, 0x274, t49); // 0x00658314
    wf(obj, 0x278, t57); // 0x0065831C
    wf(obj, 0x240, t61); // 0x0065832A
    wf(obj, 0x244, t62); // 0x00658332
    wf(obj, 0x248, t67); // 0x0065833A
    wf(obj, 0x280, t54); // 0x00658342
    wf(obj, 0x284, t49); // 0x0065834A
    wf(obj, 0x288, t57); // 0x00658352
    let t76: f32 = rf(obj, 0x210); // 0x00658378
    let t77: f32 = f32::from_bits(fadd_c(t58, t76)); // 0x00658378 addss
    let t78: f32 = f32::from_bits(fadd_c(t77, t72)); // 0x0065838C addss
    wf(obj, 0x210, t78); // 0x00658390
    let t79: f32 = rf(obj, 0x214); // 0x00658398
    let t80: f32 = f32::from_bits(fadd_c(t79, t59)); // 0x006583A0 addss
    let t81: f32 = f32::from_bits(fadd_c(t80, t66)); // 0x006583A4 addss
    wf(obj, 0x214, t81); // 0x006583A8
    let t82: f32 = rf(obj, 0x218); // 0x006583B0
    let t83: f32 = f32::from_bits(fadd_c(t82, t60)); // 0x006583B8 addss
    let t84: f32 = f32::from_bits(fadd_c(t83, t68)); // 0x006583BC addss
    wf(obj, 0x218, t84); // 0x006583C0
    let t85: f32 = fxor(t58, g_sign); // 0x006583D0
    let t86: f32 = fxor(t59, g_sign); // 0x006583D3
    let t87: f32 = fxor(t60, g_sign); // 0x006583D6
    let t88: f32 = rf(obj, 0x220); // 0x006583D9
    let t89: f32 = f32::from_bits(fadd_c(t88, t85)); // 0x006583E1 addss
    let t90: f32 = f32::from_bits(fadd_c(t89, t72)); // 0x006583E5 addss
    wf(obj, 0x220, t90); // 0x006583E9
    let t91: f32 = rf(obj, 0x224); // 0x006583F1
    let t92: f32 = f32::from_bits(fadd_c(t91, t86)); // 0x006583F9 addss
    let t93: f32 = f32::from_bits(fadd_c(t92, t66)); // 0x006583FD addss
    wf(obj, 0x224, t93); // 0x00658401
    let t94: f32 = rf(obj, 0x228); // 0x00658409
    let t95: f32 = f32::from_bits(fadd_c(t94, t87)); // 0x00658411 addss
    let t96: f32 = f32::from_bits(fadd_c(t95, t68)); // 0x00658415 addss
    wf(obj, 0x228, t96); // 0x00658419
    let t97: f32 = fxor(t66, g_sign); // 0x00658429
    let t98: f32 = fxor(t72, g_sign); // 0x0065842C
    let t99: f32 = fxor(t68, g_sign); // 0x0065842F
    let t100: f32 = rf(obj, 0x230); // 0x00658432
    let t101: f32 = f32::from_bits(fadd_c(t100, t85)); // 0x0065843A addss
    let t102: f32 = f32::from_bits(fadd_c(t101, t98)); // 0x0065843E addss
    wf(obj, 0x230, t102); // 0x00658442
    let t103: f32 = rf(obj, 0x234); // 0x0065844A
    let t104: f32 = f32::from_bits(fadd_c(t103, t86)); // 0x00658452 addss
    let t105: f32 = f32::from_bits(fadd_c(t104, t97)); // 0x00658456 addss
    wf(obj, 0x234, t105); // 0x0065845A
    let t106: f32 = rf(obj, 0x238); // 0x00658462
    let t107: f32 = f32::from_bits(fadd_c(t106, t87)); // 0x0065846A addss
    let t108: f32 = f32::from_bits(fadd_c(t107, t99)); // 0x0065846E addss
    wf(obj, 0x238, t108); // 0x00658472
    let t109: f32 = rf(obj, 0x240); // 0x0065847A
    let t110: f32 = f32::from_bits(fadd_c(t109, t58)); // 0x00658482 addss
    let t111: f32 = rf(obj, 0x244); // 0x00658488
    let t112: f32 = f32::from_bits(fadd_c(t59, t111)); // 0x00658488 addss
    let t113: f32 = f32::from_bits(fadd_c(t110, t98)); // 0x00658490 addss
    let t114: f32 = f32::from_bits(fadd_c(t112, t97)); // 0x00658494 addss
    wf(obj, 0x240, t113); // 0x00658498
    let t115: f32 = rf(obj, 0x248); // 0x006584A9
    let t116: f32 = f32::from_bits(fadd_c(t60, t115)); // 0x006584A9 addss
    wf(obj, 0x244, t114); // 0x006584B1
    let t117: f32 = f32::from_bits(fadd_c(t116, t99)); // 0x006584B9 addss
    wf(obj, 0x248, t117); // 0x006584BD
    let t118: f32 = rf(obj, 0x250); // 0x006584C5
    let t119: f32 = f32::from_bits(fadd_c(t118, t70)); // 0x006584D3 addss
    let t120: f32 = f32::from_bits(fadd_c(t119, t74)); // 0x006584EE addss
    let t121: f32 = fxor(t70, g_sign); // 0x006584F4
    let t122: f32 = fxor(t71, g_sign); // 0x006584FA
    wf(obj, 0x250, t120); // 0x00658500
    let t123: f32 = rf(obj, 0x254); // 0x0065850B
    let t124: f32 = f32::from_bits(fadd_c(t71, t123)); // 0x0065850B addss
    let t125: f32 = fxor(t73, g_sign); // 0x00658513
    let t126: f32 = f32::from_bits(fadd_c(t124, t75)); // 0x00658516 addss
    wf(obj, 0x254, t126); // 0x0065851C
    let t127: f32 = rf(obj, 0x258); // 0x00658527
    let t128: f32 = f32::from_bits(fadd_c(t73, t127)); // 0x00658527 addss
    let t129: f32 = f32::from_bits(fadd_c(t128, t69)); // 0x0065852F addss
    wf(obj, 0x258, t129); // 0x00658535
    let t130: f32 = rf(obj, 0x260); // 0x00658540
    let t131: f32 = f32::from_bits(fadd_c(t121, t130)); // 0x00658540 addss
    let t132: f32 = f32::from_bits(fadd_c(t131, t74)); // 0x00658548 addss
    wf(obj, 0x260, t132); // 0x0065854E
    let t133: f32 = rf(obj, 0x264); // 0x00658556
    let t134: f32 = f32::from_bits(fadd_c(t133, t122)); // 0x0065855E addss
    let t135: f32 = f32::from_bits(fadd_c(t134, t75)); // 0x00658562 addss
    wf(obj, 0x264, t135); // 0x00658568
    let t136: f32 = rf(obj, 0x268); // 0x00658570
    let t137: f32 = f32::from_bits(fadd_c(t136, t125)); // 0x00658578 addss
    let t138: f32 = f32::from_bits(fadd_c(t137, t69)); // 0x0065857C addss
    wf(obj, 0x268, t138); // 0x00658582
    let t139: f32 = rf(obj, 0x274); // 0x00658590
    let t140: f32 = f32::from_bits(fadd_c(t122, t139)); // 0x00658590 addss
    let t141: f32 = rf(obj, 0x278); // 0x00658598
    let t142: f32 = f32::from_bits(fadd_c(t125, t141)); // 0x00658598 addss
    let t143: f32 = fxor(t74, g_sign); // 0x006585A0
    let t144: f32 = fxor(t75, g_sign); // 0x006585AF
    let t145: f32 = f32::from_bits(fadd_c(t140, t144)); // 0x006585BE addss
    let t146: f32 = fxor(t69, g_sign); // 0x006585C4
    let t147: f32 = rf(obj, 0x270); // 0x006585CD
    let t148: f32 = f32::from_bits(fadd_c(t142, t146)); // 0x006585D5 addss
    let t149: f32 = f32::from_bits(fadd_c(t147, t121)); // 0x006585DB addss
    wf(obj, 0x274, t145); // 0x006585DF
    wf(obj, 0x278, t148); // 0x006585E7
    let t150: f32 = f32::from_bits(fadd_c(t149, t143)); // 0x006585EF addss
    wf(obj, 0x270, t150); // 0x006585F5
    let t151: f32 = rf(obj, 0x284); // 0x006585FD
    let t152: f32 = rf(obj, 0x280); // 0x00658605
    let t153: f32 = f32::from_bits(fadd_c(t70, t152)); // 0x00658605 addss
    let t154: f32 = f32::from_bits(fadd_c(t151, t71)); // 0x0065860D addss
    let t155: f32 = f32::from_bits(fadd_c(t153, t143)); // 0x00658611 addss
    let t156: f32 = f32::from_bits(fadd_c(t154, t144)); // 0x00658617 addss
    wf(obj, 0x280, t155); // 0x0065861D
    wf(obj, 0x284, t156); // 0x00658625
    let t157: f32 = rf(obj, 0x288); // 0x0065862D
    let t158: f32 = f32::from_bits(fadd_c(t157, t73)); // 0x00658635 addss
    let t159: f32 = f32::from_bits(fadd_c(t158, t146)); // 0x00658639 addss
    wf(obj, 0x288, t159); // 0x0065863F
    0
});
