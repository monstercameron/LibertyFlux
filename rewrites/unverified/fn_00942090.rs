// original: 0x00942090 streaming_wordlist_append_b (proposed)

/// Validate the id, then append its low word to the lane's second list.
///
/// A zero low word returns without doing anything (leaving `eax`
/// untouched). Otherwise the id is validated through the checker call,
/// then its low 16 bits are appended to the word list of lane `n` (read
/// at `this + 0x2040`): the list base at lane base `+0x206c`, the count
/// word at `+0x2070`, each lane record 8 bytes apart. Returns the new
/// count in `eax`.
///
/// Original: 0x00942090 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_00942090(this: u32, arg: u32) -> u32 {
    unsafe {
        const LANE: u32 = 0x2040;
        const LANE_STRIDE: u32 = 8;
        const LIST_BASE: u32 = 0x206C;
        const LIST_COUNT: u32 = 0x2070;
        const CALLEE: u32 = 1;
        if arg as u16 == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE, u32, arg & 0xFFFF);
        let lane = ((this + LANE) as *const u32).read_unaligned();
        let rec = this + lane.wrapping_mul(LANE_STRIDE);
        let count_at = (rec + LIST_COUNT) as *mut u16;
        let count = count_at.read_unaligned() as u32;
        let base = ((rec + LIST_BASE) as *const u32).read_unaligned();
        ((base + count.wrapping_mul(2)) as *mut u16).write_unaligned(arg as u16);
        let new_count = count.wrapping_add(1);
        count_at.write_unaligned(new_count as u16);
        new_count
    }
});
