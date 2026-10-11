// original: 0x009B0910 audPoliceScanner::~audPoliceScanner__deleting

/// Calls the scanner destructor, conditionally sends the object to the engine
/// notification hook when bit zero of the destructor flag is set, and returns
/// the original object pointer. The object is in ECX; the flag is one
/// callee-cleaned stack word.
lf_checker_rt::export!(thiscall, rw_009b0910(this: u32, flags: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
