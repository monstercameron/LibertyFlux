// original: 0x0069D4C0 rage::crAnimChannelRleInt::init_guarded

/// Initializes an RLE integer channel in place, relocating its pointers.
///
/// The stack arguments are the channel object and a base value. A null
/// object returns immediately (entry-`eax` residue, unverifiable, so the
/// contract only exercises live objects). Otherwise the object is stamped
/// with the RleInt vtable and each of the two pointers at `+8` and `+0x10`,
/// when non-null, is rebased by adding the relocator's answer (callee 1,
/// thiscall: base, old pointer). Returns the second answer, or 0 when the
/// second pointer was null.
///
/// Original: 0x0069D4C0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0069D4C0(obj: u32, base: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE3EBC;
        const PTR0_OFF: u32 = 8;
        const PTR1_OFF: u32 = 0x10;
        const RELOC: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if obj == 0 {
            return 0; // placeholder: original returns entry-eax residue (unverifiable, uncovered)
        }
        wr32(obj, lf_checker_rt::relocated(VTABLE));
        let p0 = rd32(obj + PTR0_OFF);
        if p0 != 0 {
            let d = lf_checker_rt::callee_thiscall!(RELOC, u32, base, p0);
            wr32(obj + PTR0_OFF, p0.wrapping_add(d));
        }
        let p1 = rd32(obj + PTR1_OFF);
        if p1 != 0 {
            let d = lf_checker_rt::callee_thiscall!(RELOC, u32, base, p1);
            wr32(obj + PTR1_OFF, p1.wrapping_add(d));
            return d;
        }
        0
    }
});
