// original: 0x00c18dd0 strlen_alloc_widen

/// Widen a string to UTF-16 and store it through command 6.
///
/// Measures the second argument string, allocates twice its length plus two
/// bytes (the multiply saturates to `0xFFFFFFFF` on overflow), widens it
/// with MultiByteToWideChar, then stores the result with command 6
/// (arguments: the first argument passed through, 1, the wide buffer, its
/// byte length) and frees the buffer. Returns 0 when the allocation fails or
/// the widening converts nothing, else command 6's answer. The saturation
/// path needs a 2 GB string and is untestable; the tested lengths are 0 to 3.
///
/// Original: 0x00C18DD0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c18dd0(this: u32, a0: u32, s: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const WIDEN_CALLEE: u32 = 2;
        const FREE: u32 = 3;
        const CMD6: u32 = 4;
        const WIDEN_SLOT: u32 = 0x00e7_3270;
        let _ = WIDEN_CALLEE;
        let mut len = 0u32;
        while ((s + len) as *const u8).read() != 0 {
            len += 1;
        }
        let wide = (len as u64 + 1) * 2;
        let n = if wide > 0xffff_ffffu64 {
            0xffff_ffffu32
        } else {
            wide as u32
        };
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, n);
        if buf == 0 {
            return 0;
        }
        let slot = lf_checker_rt::relocated(WIDEN_SLOT) as *const u32;
        let widen: extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot.read() as usize);
        let count = len + 1;
        let converted = widen(0, 0, s, 0xffff_ffff, buf, count);
        if converted == 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, buf);
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(CMD6, u32, this, a0, 1, buf, count.wrapping_add(count));
        lf_checker_rt::callee_cdecl!(FREE, u32, buf);
        ok & 0xff
    }
});
