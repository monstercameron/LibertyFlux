// original: 0x00dd9fa0 uimontageclip_apply_negated_pair (proposed)

/// Apply one flagged on/off state to a pair of child clips, refreshing each
/// child's cached 24-byte transform from the shared transform service.
///
/// `this` is the parent clip. Two child objects are read from it (first at
/// `+CHILD_A`, second at `+CHILD_B`); the low byte of `flag` selects the
/// parameter set (high bytes ignored). For each child in order:
/// 1. Call the child's virtual slot `TRANSFORM_SLOT` (thiscall, no stack
///    arguments); the returned pointer's first word points at an entry whose
///    own first word is the destination record.
/// 2. Call the transform service (callee 2, thiscall: ECX = scratch buffer,
///    two stack words = a selector constant and zero); it answers a pointer
///    to 24 bytes.
/// 3. Copy those 24 bytes to the destination record at `+COPY_DST`.
/// 4. Call the release helper (callee 3, thiscall, ECX = scratch buffer).
///
/// The selector constants are negative floats here (this function's twin at
/// 0x00dda0c0 uses the same magnitudes with positive sign): `SEL_A_ON`,
/// `SEL_B_ON` when the flag byte is non-zero, `SEL_A_OFF`, `SEL_B_OFF` when
/// it is zero. The scratch buffer is never read back by this function; both
/// service calls in each block reuse it. Finally the flag byte is stored at
/// `+FLAG_OFF`.
///
/// Original: 0x00dd9fa0 (thiscall, one stack word, the callee pops 4 bytes, no return value).
lf_checker_rt::export!(thiscall, rw_00dd9fa0(this: u32, flag: u32) -> u32 {
    unsafe {
        const CHILD_A: u32 = 0x30c;
        const CHILD_B: u32 = 0x304;
        const FLAG_OFF: u32 = 0x319;
        const TRANSFORM_SLOT: u32 = 0xf8;
        const COPY_DST: u32 = 0x10;
        const COPY_WORDS: u32 = 6;
        const SEL_A_ON: u32 = 0xc2040000;
        const SEL_A_OFF: u32 = 0xc1200000;
        const SEL_B_ON: u32 = 0xc2100000;
        const SEL_B_OFF: u32 = 0xc1500000;
        const SVC_CALLEE: u32 = 2;
        const RELEASE_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall_transform(child: u32) -> u32 {
            unsafe {
                let vt = rd32(child);
                let slot = rd32(vt + TRANSFORM_SLOT);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(child)
            }
        }
        #[inline(always)]
        unsafe fn copy24(dst: u32, src: u32) {
            unsafe {
                let mut i = 0u32;
                while i < COPY_WORDS {
                    let v = rd32(src + i * 4);
                    (dst as *mut u32).add(i as usize).write_unaligned(v);
                    i += 1;
                }
            }
        }

        let on = (flag as u8) != 0;
        let mut scratch = [0u32; 8];
        let buf = scratch.as_mut_ptr() as u32;

        let dest_a = rd32(rd32(vcall_transform(rd32(this + CHILD_A))));
        let src_a: u32 = lf_checker_rt::callee_thiscall!(
            SVC_CALLEE, u32, buf, if on { SEL_A_ON } else { SEL_A_OFF }, 0);
        copy24(dest_a + COPY_DST, src_a);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, buf);

        let dest_b = rd32(rd32(vcall_transform(rd32(this + CHILD_B))));
        let src_b: u32 = lf_checker_rt::callee_thiscall!(
            SVC_CALLEE, u32, buf, if on { SEL_B_ON } else { SEL_B_OFF }, 0);
        copy24(dest_b + COPY_DST, src_b);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, buf);

        ((this + FLAG_OFF) as *mut u8).write(flag as u8);
        0
    }
});
