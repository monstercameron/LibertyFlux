// original: 0x00c562f0 CEventLeaderEnteredCarAsDriver::~CEventLeaderEnteredCarAsDriver
/// Clone a leader-entered-car event. Allocate a new event, invoke its base constructor, copy the referenced vehicle pointer from +0x0c, and stamp the derived vtable. If the pointer is non-null, forward the address of the new field to the reference helper. Return zero when allocation fails and otherwise return the new object.
lf_checker_rt::export!(thiscall, rw_00c562f0(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const EVENT_VTABLE_VA: u32 = 0x00ec_ae04;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let referenced_vehicle = core::ptr::read_unaligned(this.wrapping_add(0x0c) as *const u32);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh);
    let reference_field = fresh.wrapping_add(0x0c);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    core::ptr::write_unaligned(reference_field as *mut u32, referenced_vehicle);
    if referenced_vehicle != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, referenced_vehicle, reference_field);
    }
    fresh
}
});
