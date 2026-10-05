// original: 0x00ABF290 stream_insert_timed_key (proposed)

/// Insert a timed key (`id`, `time`) into the streaming key set.
///
/// The original passes the address of two spill slots holding the id word
/// and the time float to the insert callee, with the key-set object
/// (cdecl, two words). The rewrite passes equivalent locals; the call
/// comparison skips the addresses and compares the pointed-to words.
/// The callee's out-param write lands in uncompared scratch. No value
/// is returned.
lf_checker_rt::export!(cdecl, rw_00ABF290(id: u32, time: u32) -> u32 {
    unsafe {
        const KEYSET: u32 = 0x0150E278;
        const INSERT: u32 = 1;
        let mut wid = id;
        let mut wtime = time;
        lf_checker_rt::callee_thiscall!(
            INSERT,
            u32,
            KEYSET,
            &mut wid as *mut u32 as u32,
            &mut wtime as *mut u32 as u32
        );
        0
    }
});
