// original: 0x00c55620 CEventAcquaintancePedRespect::~CEventAcquaintancePedRespect__deleting
/// Install the derived event vtable and run the recorded base teardown. If option bit zero is set, call the global release helper with the manager pointer and this object. Return this; the thiscall consumes one stack option word.
lf_checker_rt::export!(thiscall, rw_00c55620(this: u32, options: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x016669c8;
    const BASE_TEARDOWN: u32 = 1;
    const GLOBAL_RELEASE: u32 = 2;
    const DELETE_FLAG: u32 = 0x01;
    unsafe {
        core::ptr::write_unaligned(this as *mut u32, lf_checker_rt::relocated(0x00ecabc4));
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
        if options & DELETE_FLAG != 0 {
            let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA));
            let _: u32 = lf_checker_rt::callee_thiscall!(GLOBAL_RELEASE, u32, manager, this);
        }
    }
    this
});
