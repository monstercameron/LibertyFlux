// original: 0x0089cd40 audio_hash_then_work (proposed)

/// Hashes the first argument, then runs the worker on the hash plus reordered inputs.
///
/// Calls the hash helper (callee 1, cdecl/2) with (`a0`, 0), then the worker
/// (callee 2, thiscall/4 on `this`) with the hash first and the three
/// original arguments reordered as (`a1`, `a2`, `a0`), returning its result.
///
/// Original: 0x0089cd40 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0089cd40(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HASH: u32 = 1;
        const WORKER: u32 = 2;
        let h: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, a0, 0);
        lf_checker_rt::callee_thiscall!(WORKER, u32, this, h, a1, a2, a0)
    }
});
