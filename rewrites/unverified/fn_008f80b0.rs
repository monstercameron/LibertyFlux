// original: 0x008f80b0 input_poll_gate (proposed)

/// Poll the sampler unless both gate bytes are clear.
///
/// The entry ECX is threaded into the sync callee (thiscall, no stack
/// words). A fixed object then runs the broadcast callee, the flag callee
/// runs with 0, and the query callee runs with the fixed object; between
/// them the global flag byte is set to 1 and cleared again. Two global gate
/// bytes are read: when both are zero the function returns the query
/// answer. Otherwise the global float is split across overlapping scratch
/// slots: the first float callee runs with (0, 0) and then with the
/// float's high half-word and the second gate byte ORed with the float's
/// low half-word (the float store overlaps both words), and the second
/// float callee runs with an uninitialized scratch word, which is 0 under
/// the checker's zero stack fill; its answer is returned.
///
/// Thiscall: thread-through ECX, no stack words. The contract fills
/// unwritten scratch with zero, matching the original's reads of it.
lf_checker_rt::export!(thiscall, rw_008f80b0(this: u32) -> u32 {
    unsafe {
        const C_SYNC: u32 = 1;
        const C_BCAST: u32 = 2;
        const C_FLAG: u32 = 3;
        const C_QUERY: u32 = 4;
        const C_F2: u32 = 5;
        const C_F1: u32 = 6;
        const G_FLAG: u32 = 0x118dc60;
        const FIXED_OBJ: u32 = 0x118d7f0;
        const G_GATES: u32 = 0x18b6ed4;
        const G_FLOAT: u32 = 0x106c31c;
        ((lf_checker_rt::global::<u8>(G_FLAG)) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_SYNC, u32, this);
        let fixed = lf_checker_rt::relocated(FIXED_OBJ);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_BCAST, u32, fixed);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_FLAG, u32, 0);
        ((lf_checker_rt::global::<u8>(G_FLAG)) as *mut u8).write(0);
        let r4: u32 = lf_checker_rt::callee_thiscall!(C_QUERY, u32, fixed);
        let gates = (lf_checker_rt::global::<u32>(G_GATES)).read_unaligned();
        let clo = (gates & 0xff) as u8;
        let ahi = ((gates >> 16) & 0xff) as u8;
        if ahi == 0 && clo == 0 {
            return r4;
        }
        let f = (lf_checker_rt::global::<u32>(G_FLOAT)).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(C_F2, u32, 0, 0);
        let _: u32 =
            lf_checker_rt::callee_cdecl!(C_F2, u32, (f >> 16) & 0xffff, (f & 0xffff0000) | clo as u32);
        lf_checker_rt::callee_cdecl!(C_F1, u32, 0)
    }
});
