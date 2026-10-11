// original: 0x00c56620 CEventScriptedAttractor::~CEventScriptedAttractor
/// Clone a scripted-attractor event. Allocate a new object, forward the source fields at +0x18 and +0x1c plus a zero word to the attractor base constructor, stamp the derived vtable, and return the object. A failed allocation returns zero.
lf_checker_rt::export!(thiscall, rw_00c56620(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const EVENT_VTABLE_VA: u32 = 0x00ec_ab64;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let attractor = core::ptr::read_unaligned(this.wrapping_add(0x18) as *const u32);
    let task = core::ptr::read_unaligned(this.wrapping_add(0x1c) as *const u32);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh, attractor, task, 0);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    fresh
}
});
