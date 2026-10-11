// original: 0x00b4bc20 group_free_entry

/// Destroy the optional helper object stored at group offset 0x50, clear
/// that field when present, then remove members through the group subobject
/// at +8. The destructor is the object's vtable slot zero with argument 1;
/// member removal also receives argument 1. The group arrives in ECX.
lf_checker_rt::export!(thiscall, rw_00B4BC20(group: u32) -> u32 {
    const HELPER_FIELD: u32 = 0x50;
    const VTABLE_FIELD: u32 = 0;
    const DESTRUCTOR_SLOT: u32 = 0;
    const MEMBER_SUBOBJECT: u32 = 0x08;
    const DESTRUCTOR_MODE: u32 = 1;
    const REMOVE_MODE: u32 = 1;
    type Destructor = unsafe extern "thiscall" fn(u32, u32) -> u32;

    // SAFETY: the supplied group object covers the helper pointer field.
    let helper = unsafe {
        (group.wrapping_add(HELPER_FIELD) as *const u32).read_unaligned()
    };
    if helper != 0 {
        // SAFETY: the helper object begins with its vtable pointer.
        let vtable = unsafe { (helper.wrapping_add(VTABLE_FIELD) as *const u32).read_unaligned() };
        // SAFETY: the vtable slot is planted with a checker recorder stub.
        let target = unsafe { (vtable.wrapping_add(DESTRUCTOR_SLOT) as *const u32).read_unaligned() };
        let destructor: Destructor = unsafe { core::mem::transmute(target as usize) };
        // SAFETY: contract slot zero records the original thiscall and one argument.
        let _ignored = unsafe { destructor(helper, DESTRUCTOR_MODE) };
        // SAFETY: clear the helper pointer after the destructor call.
        unsafe { (group.wrapping_add(HELPER_FIELD) as *mut u32).write_unaligned(0) };
    }

    lf_checker_rt::callee_thiscall!(2, u32,
        group.wrapping_add(MEMBER_SUBOBJECT), REMOVE_MODE)
});
