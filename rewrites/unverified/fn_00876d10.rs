// original: 0x00876D10 rebase_optional_frame_sample_bounds

/// Rebase an optional frame-sample bounds record.
///
/// A null record pointer is a no-op. Otherwise the function forwards the
/// record pointer and the caller's adjustment word to the bounds rebase
/// helper, preserving their calling positions: the record is the this
/// pointer and the adjustment is its one stack argument. The wrapper does not
/// consume a helper result; caller-visible EAX residue is outside this model.
///
/// Original calling convention: cdecl with two stack arguments; no value is
/// returned.
lf_checker_rt::export!(cdecl, rw_00876d10(record: u32, adjustment: u32) -> () {
    if record != 0 {
        let _helper_result = lf_checker_rt::callee_thiscall!(1, u32, record, adjustment);
    }
});
