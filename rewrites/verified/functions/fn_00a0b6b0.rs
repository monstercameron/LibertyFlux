// original: 0x00a0b6b0 cleanup_lookup_valid (proposed)
/// Look up the cleanup record for a descriptor and test it strictly.
///
/// Resolves `item` through the typed dispatcher (flag word 1); a null
/// record is invalid. Otherwise the record is validity-checked with the
/// strict flag set. Returns 1 when both steps succeed (low byte only).
/// Thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00a0b6b0(this: u32, item: u32) -> u32 {
    unsafe {
        const DISPATCH: u32 = 0;
        const VALIDATE: u32 = 1;
        let rec = lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, item, 1u32);
        if rec == 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this, rec, 1u32);
        ((ok & 0xff) != 0) as u32
    }
});
