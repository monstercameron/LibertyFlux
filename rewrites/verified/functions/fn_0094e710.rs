// original: 0x0094E710 fill_live_records_wide (proposed)

/// Fill 100 wide records through callee 3, gating idle ones to a stack struct.
///
/// For each of the 100 records of 0x30 bytes at `this + 8`: when both the
/// word at record - 4 and the word at record + 0 are zero the record is
/// idle and the stack struct is used at once. Otherwise callee 1 runs with
/// (index, `KIND`) and callee 2 as a thiscall with the context word `CTX`
/// in ECX and (`SUBKIND`, callee 1's answer) on the stack; only the LOW
/// byte of callee 2's answer is tested. A non-zero byte selects the record
/// itself minus 8 as the fill source, a zero byte the stack struct whose 12
/// snapshot words read (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0). Callee 3 runs
/// with (source, 0x30) either way; the source pointer's value differs per
/// side so the contract skips it and snapshots the 12 words. Ends with the
/// stack-cookie check as an intercepted call whose cookie is uncompared
/// (`ret: preserve`), and returns 1 in the low byte.
///
/// Original: 0x0094E710 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094E710(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 100;
        const STRIDE: u32 = 0x30;
        const KIND: u32 = 0xA;
        const CTX: u32 = 0x12BD0C4;
        const SUBKIND: u32 = 0xC;
        const LOOKUP: u32 = 1;
        const CHECK: u32 = 2;
        const FILL: u32 = 3;
        const COOKIE: u32 = 4;
        let ctx = (lf_checker_rt::global::<u32>(CTX) as *const u32).read();
        let st = [1u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut i = 0u32;
        while i < COUNT {
            let rec = this
                .wrapping_add(8)
                .wrapping_add(i.wrapping_mul(STRIDE));
            let prev = (rec.wrapping_sub(4) as *const u32).read_unaligned();
            let cur = (rec as *const u32).read_unaligned();
            let src = if prev != 0 || cur != 0 {
                let r = lf_checker_rt::callee_cdecl!(LOOKUP, u32, i, KIND);
                let ok = lf_checker_rt::callee_thiscall!(CHECK, u32, ctx, SUBKIND, r);
                if (ok & 0xFF) != 0 {
                    rec.wrapping_sub(8)
                } else {
                    st.as_ptr() as u32
                }
            } else {
                st.as_ptr() as u32
            };
            lf_checker_rt::callee_cdecl!(FILL, u32, src, 0x30);
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(COOKIE, u32, 0);
        1
    }
});
