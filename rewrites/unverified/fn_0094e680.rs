// original: 0x0094E680 fill_gated_records (proposed)

/// Fill 16 records through callee 3, choosing a heap or stack source each.
///
/// For each of the 16 records of 0x20 bytes at `this`: when the SIGNED tag
/// at record + 4 is non-negative, callee 1 runs with (index, `KIND`) and
/// callee 2 as a thiscall with the context word `CTX` in ECX and
/// (`SUBKIND`, callee 1's answer) on the stack; only the LOW byte of callee
/// 2's answer is tested. When the tag is negative or that byte is zero, a
/// stack struct (-1, then zeros) is the fill source instead of the record.
/// Callee 3 runs with (source, 0x20) either way; the source pointer's value
/// differs per side so the contract skips it and snapshots the 8 words.
/// Ends with the stack-cookie check as an intercepted call whose cookie is
/// uncompared (`ret: preserve`), and returns 1 in the low byte.
///
/// Original: 0x0094E680 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094E680(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x20;
        const KIND: u32 = 9;
        const CTX: u32 = 0x12BD0C4;
        const SUBKIND: u32 = 0xD;
        const LOOKUP: u32 = 1;
        const CHECK: u32 = 2;
        const FILL: u32 = 3;
        const COOKIE: u32 = 4;
        let ctx = (lf_checker_rt::global::<u32>(CTX) as *const u32).read();
        let st = [0xFFFF_FFFFu32, 0, 0, 0, 0, 0, 0, 0];
        let mut i = 0u32;
        while i < COUNT {
            let rec = this.wrapping_add(i.wrapping_mul(STRIDE));
            let tag = (rec.wrapping_add(4) as *const u32).read_unaligned();
            let use_heap = if (tag as i32) >= 0 {
                let r = lf_checker_rt::callee_cdecl!(LOOKUP, u32, i, KIND);
                let ok = lf_checker_rt::callee_thiscall!(CHECK, u32, ctx, SUBKIND, r);
                (ok & 0xFF) != 0
            } else {
                false
            };
            if use_heap {
                lf_checker_rt::callee_cdecl!(FILL, u32, rec, 0x20);
            } else {
                lf_checker_rt::callee_cdecl!(FILL, u32, st.as_ptr() as u32, 0x20);
            }
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(COOKIE, u32, 0);
        1
    }
});
