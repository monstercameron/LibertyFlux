// original: 0x00e67e30 bind_model_n_polmav (proposed)

//! r-s270 rewrite (see crate lf_rs270_rw for the verified build).

/// Bind the model name "n_polmav" to its registration slot.
///
/// Forwards two relocated constants to the shared thiscall registrar: the
/// slot address in ECX and the name-string address as the one stack word,
/// and returns the registrar's result unchanged.
///
/// `NAME` is a NUL-terminated string in read-only data; `SLOT` is the
/// writable registration slot for this model. The original takes no
/// arguments and reads no memory itself (both values are immediates), so
/// every trial differs only in the scripted registrar answer, which must
/// pass through bit-exact, including all 32 bits of any value.
///
/// Original: 0x00e67e30 (no arguments, plain `ret`; declared cdecl).
lf_checker_rt::export!(cdecl, rw_00e67e30() -> u32 {
    const NAME: u32 = 0x00e9e674;
    const SLOT: u32 = 0x012fa370;
    const REGISTRAR: u32 = 1;
    unsafe {
        lf_checker_rt::callee_thiscall!(REGISTRAR, u32,
            lf_checker_rt::relocated(SLOT), lf_checker_rt::relocated(NAME))
    }
});
