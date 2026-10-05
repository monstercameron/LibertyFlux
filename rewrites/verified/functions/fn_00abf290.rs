// original: 0x00ABF290 stream_insert_timed_key (proposed)

/// Insert a timed key (`id`, `time`) into the streaming key set.
///
/// The original passes two pointers to the insert callee: one to an
/// uninitialized spill slot (the out slot) and one to a two-word id/time
/// pair (cdecl, two words). The rewrite passes an equivalent zero word (the
/// contract's zero stack fill makes the original's spill deterministic) and
/// its own id/time pair; the call comparison skips the addresses and
/// compares the pointed-to words. The callee's out-param write lands in
/// uncompared scratch. No value is returned.
lf_checker_rt::export!(cdecl, rw_00ABF290(id: u32, time: u32) -> u32 {
    unsafe {
        const KEYSET: u32 = 0x0150E278;
        const INSERT: u32 = 1;
        let mut spill: u32 = 0;
        let mut pair = [id, time];
        lf_checker_rt::callee_thiscall!(
            INSERT,
            u32,
            lf_checker_rt::relocated(KEYSET),
            &mut spill as *mut u32 as u32,
            pair.as_mut_ptr() as u32
        );
        0
    }
});
