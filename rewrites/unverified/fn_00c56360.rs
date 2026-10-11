// original: 0x00c56360 CEventLeaderExitedCarAsDriver::~CEventLeaderExitedCarAsDriver
/// Clone the CEventLeaderExitedCarAsDriver event. Allocate an event object through the global manager; a null result returns zero. Otherwise call the base event constructor, stamp the derived vtable, and return the allocated object.
lf_checker_rt::export!(thiscall, rw_00c56360(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const EVENT_VTABLE_VA: u32 = 0x00ecae5c;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    fresh
}
});
