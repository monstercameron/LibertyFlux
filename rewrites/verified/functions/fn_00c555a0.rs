// original: 0x00c555a0 CEntityScanner::vf0
/// Run the CEntityScanner base teardown without stack arguments. When option bit zero is set, pass this object to the shared cdecl release helper, then return this. The thiscall consumes one stack word.
lf_checker_rt::export!(thiscall, rw_00c555a0(this: u32, options: u32) -> u32 {
    const BASE_TEARDOWN: u32 = 1;
    const OPTIONAL_RELEASE: u32 = 2;
    const RELEASE_FLAG: u32 = 0x01;
    let _: u32 = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
    if options & RELEASE_FLAG != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(OPTIONAL_RELEASE, u32, this);
    }
    this
});
