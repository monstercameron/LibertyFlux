// original: 0x00c558c0 CPedScanner::vf0
/// Run the CPedScanner base teardown and, when option bit zero is set, pass this object to the cdecl release helper. Return this unchanged. The caller-supplied option word is callee-cleaned by this method.
lf_checker_rt::export!(thiscall, rw_00c558c0(this: u32, options: u32) -> u32 {
let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
if options & 1 != 0 {
    let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, this);
}
this
});
