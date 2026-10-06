// original: 0x00dd9230 UIBasicClip::vf0

/// Call the helper with a fixed argument.
///
/// The object pointer in ECX is ignored. The fixed word `CONST_ARG` (a file
/// address, relocated in the worker) is pushed and helper 1 is called with
/// the cdecl convention (caller pops the word). The helper's answer in EAX
/// is the result; nothing else is read or written.
///
/// Original: thiscall shape with no stack arguments, one cdecl call of one
/// word, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dd9230(this: u32) -> u32 {
    const CONST_ARG: u32 = 0x00ef_bf70;
    unsafe { lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CONST_ARG)) }
});
