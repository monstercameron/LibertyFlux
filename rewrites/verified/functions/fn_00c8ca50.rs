// original: 0x00c8ca50 audio_bank_lookup (proposed)
///
/// Looks `key` up in a three-entry global table (file address table:
/// dword key, half-word weight, 8 bytes per entry). Scans entries 0..2:
/// on a key match, writes `weight + *acc` to `*out` and returns 1,
/// otherwise adds the weight into `*acc` (wrapping) and continues; with
/// no match returns 0. Cdecl, three stack arguments
/// (key, acc pointer, out pointer); returns a byte in al.

lf_checker_rt::export!(cdecl, rw_00c8ca50(key: u32, acc: u32, out: u32) -> u8 {
    unsafe {
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        const TABLE: u32 = 0x1050a94;
        const PITCH: u32 = 8;
        const COUNT: u32 = 3;
        let table = lf_checker_rt::relocated(TABLE);
        let mut i: u32 = 0;
        while i < COUNT {
            let e = table.wrapping_add(i.wrapping_mul(PITCH));
            if rd32(e) == key {
                let w = rd16(e.wrapping_add(4));
                let sum = w.wrapping_add((acc as *const u16).read_unaligned());
                (out as *mut u16).write_unaligned(sum);
                return 1;
            }
            let w = rd16(e.wrapping_add(4));
            let a = acc as *mut u16;
            a.write_unaligned(a.read_unaligned().wrapping_add(w));
            i += 1;
        }
        0
    }
});
