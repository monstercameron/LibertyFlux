// original: 0x00c05fe0 stream_set_clear (proposed)

/// Release every entry of the streaming set and reset its header.
///
/// `this` points to the set. The header fields at `F0`, `F1`, `F2`, `F3`,
/// `FB` and `F4` are cleared (with `F1` set to all-bits), every entry of the
/// array at `ITEMS` (16-bit length at `COUNT`) goes to the release helper
/// (callee 1) and its slot is cleared (the loop's entry test reads the
/// incoming `ax`, so with entry `ax` zero it runs exactly when the length is
/// non-zero; other entry values gate it), then the range helper (callee 2)
/// receives the array's start and end with the array field itself in `ecx`.
/// Finally, when the aux pointer at `AUX` is set it goes to the release
/// helper and `AUX` and `AUX2` are cleared, otherwise both are left alone.
/// Returns the last helper answer.
///
/// Original: 0x00c05fe0 (thiscall, no stack words; 1 is cdecl, 2 thiscall).
lf_checker_rt::export!(thiscall, rw_00c05fe0(this: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x30;
        const F1: u32 = 0x34;
        const F2: u32 = 0x20;
        const F3: u32 = 0x24;
        const FB: u32 = 0x00;
        const F4: u32 = 0x38;
        const ITEMS: u32 = 0x28;
        const COUNT: u32 = 0x2c;
        const AUX: u32 = 0x48;
        const AUX2: u32 = 0x4c;
        const FREE: u32 = 1;
        const RANGE: u32 = 2;
        (this.wrapping_add(F0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(F1) as *mut u32).write_unaligned(0xffff_ffff);
        (this.wrapping_add(F2) as *mut u32).write_unaligned(0);
        (this.wrapping_add(F3) as *mut u32).write_unaligned(0);
        (this as *mut u8).write(0);
        (this.wrapping_add(F4) as *mut u32).write_unaligned(0);
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let mut r = 0u32;
        if count != 0 {
            let mut i = 0u32;
            while (i as i32) < (count as i32) {
                let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
                let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
                r = lf_checker_rt::callee_cdecl!(FREE, u32, v);
                let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
                (base.wrapping_add(i.wrapping_mul(4)) as *mut u32).write_unaligned(0);
                i += 1;
            }
        }
        let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        r = lf_checker_rt::callee_thiscall!(
            RANGE, u32, this.wrapping_add(ITEMS), base, base.wrapping_add(count.wrapping_mul(4)));
        let aux = (this.wrapping_add(AUX) as *const u32).read_unaligned();
        if aux != 0 {
            r = lf_checker_rt::callee_cdecl!(FREE, u32, aux);
            (this.wrapping_add(AUX) as *mut u32).write_unaligned(0);
            (this.wrapping_add(AUX2) as *mut u32).write_unaligned(0);
        }
        r
    }
});
