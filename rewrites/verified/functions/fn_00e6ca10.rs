// original: 0x00e6ca10 EC_BUSSTOPRUS (symbols)

/// Register one fixed vehicle key against a fixed slot: `return CALLEE(this=THIS_PTR, ARG)`.
///
/// The original pushes a constant key address, loads a constant object pointer
/// and calls the shared register routine (thiscall/1, callee cleans up),
/// returning its answer. Takes no arguments.
///
/// Original: 0x00e6ca10 (cdecl shape, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6ca10() -> u32 {
    unsafe {
        const THIS_PTR: u32 = 0x01720900;
        const ARG: u32 = 0x00ee415c;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS_PTR), lf_checker_rt::relocated(ARG))
    }
});
