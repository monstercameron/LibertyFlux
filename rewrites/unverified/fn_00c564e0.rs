// original: 0x00c564e0 CEventAcquaintancePedHateBadlyLit::~CEventAcquaintancePedHateBadlyLit
/// Clone a badly-lit acquaintance-ped event. Allocate a new object, call the acquaintance-event base constructor with the referenced ped at +0x18, then copy the relation and payload fields. When the signed relation field at +0x1c is -1, replace it from the default relation global and copy four dwords from the referenced entity's block at +0x30. Return zero on allocation failure and otherwise return the new object.
lf_checker_rt::export!(thiscall, rw_00c564e0(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const DEFAULT_RELATION_VA: u32 = 0x0117_35b4;
    const EVENT_VTABLE_VA: u32 = 0x00ec_ac84;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let referenced_ped = core::ptr::read_unaligned(this.wrapping_add(0x18) as *const u32);
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh, referenced_ped);
    let relation = core::ptr::read_unaligned(this.wrapping_add(0x1c) as *const u32);
    core::ptr::write_unaligned(fresh.wrapping_add(0x1c) as *mut u32, relation);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    for offset in [0x20u32, 0x24, 0x28] {
        let value = core::ptr::read_unaligned(this.wrapping_add(offset) as *const u32);
        core::ptr::write_unaligned(fresh.wrapping_add(offset) as *mut u32, value);
    }
    if relation == u32::MAX {
        let default_relation = lf_checker_rt::global::<u32>(DEFAULT_RELATION_VA).read_unaligned();
        core::ptr::write_unaligned(fresh.wrapping_add(0x1c) as *mut u32, default_relation);
        let entity = core::ptr::read_unaligned(referenced_ped.wrapping_add(0x20) as *const u32);
        for offset in [0x30u32, 0x34, 0x38, 0x3c] {
            let value = core::ptr::read_unaligned(entity.wrapping_add(offset) as *const u32);
            core::ptr::write_unaligned(fresh.wrapping_add(offset - 0x10) as *mut u32, value);
        }
    }
    fresh
}
});
