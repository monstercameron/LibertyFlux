// original: 0x009747f0 audGtaOcclusionGroup::vf0

/// Destroy the occlusion group, freeing it when the flag's low bit is set.
///
/// Runs the member destructor, then when bit 0 of `flag` is set asks the
/// global manager to free the object. Returns the object pointer in EAX.
/// Original: 0x009747F0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009747f0(this: u32, flag: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x115FD54;
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flag & 1 != 0 {
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read_unaligned();
            lf_checker_rt::callee_thiscall!(FREE, u32, mgr, this);
        }
        this
    }
});
