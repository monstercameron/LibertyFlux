// original: 0x00c556b0 CEventLeaderEnteredCarAsDriver::~CEventLeaderEnteredCarAsDriver__deleting
/// Install the derived event vtable, release the reference stored at +0x0c when non-null, then run the base teardown. Option bit zero controls the final global release call. Return this; the thiscall consumes one stack option word.
lf_checker_rt::export!(thiscall, rw_00c556b0(this: u32, options: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x016669c8;
    const REFERENCE_SLOT: u32 = 0x0c;
    const RELEASE_REFERENCE: u32 = 1;
    const BASE_TEARDOWN: u32 = 2;
    const GLOBAL_RELEASE: u32 = 3;
    const DELETE_FLAG: u32 = 0x01;
    unsafe {
        core::ptr::write_unaligned(this as *mut u32, lf_checker_rt::relocated(0x00ecae04));
        let reference_slot = this.wrapping_add(REFERENCE_SLOT);
        let referenced_value = core::ptr::read_unaligned(reference_slot as *const u32);
        if referenced_value != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE_REFERENCE, u32, referenced_value, reference_slot);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
        if options & DELETE_FLAG != 0 {
            let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA));
            let _: u32 = lf_checker_rt::callee_thiscall!(GLOBAL_RELEASE, u32, manager, this);
        }
    }
    this
});
