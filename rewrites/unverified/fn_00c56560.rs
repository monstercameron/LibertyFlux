// original: 0x00c56560 CEventAcquaintancePedLike::~CEventAcquaintancePedLike
/// Clone the CEventAcquaintancePedLike event. Allocate an object, pass the source's referenced ped from +0x18 to the acquaintance-event base constructor, write the derived vtable, and return the new object. A failed allocation returns zero.
lf_checker_rt::export!(thiscall, rw_00c56560(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const EVENT_VTABLE_VA: u32 = 0x00ecac24;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let referenced_ped = core::ptr::read_unaligned(this.wrapping_add(0x18) as *const u32);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh, referenced_ped);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    fresh
}
});
