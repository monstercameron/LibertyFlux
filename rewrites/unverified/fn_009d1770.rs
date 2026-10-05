// original: 0x009D1770 stream_singleton_ensure (proposed)
//
/// Ensures the global streaming context exists, creating it on first use.
///
/// Returns 0 at once when the global at `0x012958AC` is already set. Else
/// allocates `0x64` bytes (callee 1); a null answer records null and reports
/// `OUTOFMEMORY`. Otherwise the constructor (callee 2) runs on the block, its
/// return is stored to the global, and a null constructor result reports
/// `OUTOFMEMORY` while anything else reports success. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_009D1770() -> u32 {
    unsafe {
        const SINGLETON: u32 = 0x012958AC;
        const SIZE: u32 = 0x64;
        const OOM: u32 = 0x8007000e;
        const NEW: u32 = 1;
        const CTOR: u32 = 2;
        let slot = lf_checker_rt::global::<u32>(SINGLETON);
        if slot.read() != 0 {
            return 0;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(NEW, u32, SIZE);
        if p == 0 {
            slot.write(0);
            return OOM;
        }
        let q: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, p);
        slot.write(q);
        if q == 0 {
            return OOM;
        }
        0
    }
});
