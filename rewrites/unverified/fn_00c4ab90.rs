// original: 0x00C4AB90 camera_cinematic_get_target_state

/// Return the result of the target object's virtual method at table offset
/// 0x0c, or zero when the receiver's target pointer is null.
///
/// The receiver is in ECX and stores the target pointer at offset 0x118. The
/// method is a no-argument thiscall whose return value is forwarded from the
/// original tail jump. The contract plants recorder stubs in the fabricated
/// target vtable, so the selected virtual call remains visible.
lf_checker_rt::export!(thiscall, rw_c4ab90(this: u32) -> u32 {
    const TARGET_FIELD: u32 = 0x118;
    const VTABLE_FIELD: u32 = 0;
    const METHOD_SLOT: u32 = 0x0c;

    let target = unsafe {
        (this.wrapping_add(TARGET_FIELD) as *const u32).read_unaligned()
    };
    if target == 0 {
        return 0;
    }
    let vtable = unsafe { (target.wrapping_add(VTABLE_FIELD) as *const u32).read_unaligned() };
    let method_address = unsafe { (vtable.wrapping_add(METHOD_SLOT) as *const u32).read_unaligned() };
    type TargetMethod = unsafe extern "thiscall" fn(u32) -> u32;
    let method: TargetMethod = unsafe { core::mem::transmute(method_address as usize) };
    unsafe { method(target) }
});
