// original: 0x00B864C0 dist_probe_b
/// Probe the table checker for the derived entity slot.
///
/// Combines two resolved bases (callees 1-2) into a slot address
/// (`base + idx*15*8`); a null slot or a closed gate (callee 3) answers
/// 0. Otherwise a frame point is filled (callee 4) and tested
/// (callee 5). Returns 1 on a hit.
///
/// Original: 0x00B864C0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00B864C0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TABLE: u32 = 0x011D78F8;
        let p1 = lf_checker_rt::callee_cdecl!(1, u32,);
        let p2 = lf_checker_rt::callee_cdecl!(2, u32,);
        let n = rd32(p1 + 0xA0);
        let base = rd32(p2 + 8);
        let slot = base.wrapping_add(n.wrapping_mul(16).wrapping_sub(n).wrapping_mul(8));
        let gate = lf_checker_rt::callee_cdecl!(3, u32,);
        if gate & 0xFF == 0 || slot == 0 {
            return 0;
        }
        let mut pt = [0u32; 4];
        lf_checker_rt::callee_cdecl!(4, u32, pt.as_mut_ptr() as u32);
        let hit = lf_checker_rt::callee_thiscall!(5, u32, TABLE, slot, pt.as_ptr() as u32);
        if hit & 0xFF != 0 { 1 } else { 0 }
    }
});
