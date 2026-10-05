// original: 0x00a4a8c0 vehicle_probe_with_struct
/// Probe the callee with a global-seeded struct; return the inverted answer.
///
/// Builds a 21-word struct from three globals (at words 4-6, 8-10, 12-14),
/// zeroes and 0xFFFF tails, then calls the worker (cdecl/6) on
/// `(obj, [obj+8]-k, 0, struct, 6, 4)` where k is 10.0 for class 4 else 2.5.
/// Returns 1 when the callee's AL is 0, else 0 (thiscall, one stack
/// argument). Only AL is compared. The struct pointer is skipped; the 14
/// initialized struct words are compared through call-time snapshots.
export!(thiscall, rw_00a4a8c0(this: u32, obj: u32) -> u32 {
    unsafe {
        let g0 = (relocated(0x01b4b320) as *const u32).read_unaligned();
        let g1 = (relocated(0x01b4b324) as *const u32).read_unaligned();
        let g2 = (relocated(0x01b4b328) as *const u32).read_unaligned();
        let class = (this.wrapping_add(0x1304) as *const u32).read_unaligned();
        let mut st = [0u32; 21];
        st[0] = 0;
        st[4] = g0;
        st[5] = g1;
        st[6] = g2;
        st[8] = g0;
        st[9] = g1;
        st[10] = g2;
        st[12] = g0;
        st[13] = g1;
        st[14] = g2;
        st[16] = 0;
        st[17] = 0;
        st[18] = 0;
        st[19] = 0xffff;
        let k = if class == 4 {
            f32::from_bits((relocated(0x00fe8b08) as *const u32).read_unaligned())
        } else {
            f32::from_bits((relocated(0x00fe8a60) as *const u32).read_unaligned())
        };
        let f = f32::from_bits((obj.wrapping_add(8) as *const u32).read_unaligned());
        let d = (core::hint::black_box(f) - core::hint::black_box(k)).to_bits();
        let r: u32 = callee_cdecl!(1, u32, obj, d, 0, (&mut st as *mut u32) as u32, 6, 4);
        if (r as u8) == 0 { 1 } else { 0 }
    }
});
