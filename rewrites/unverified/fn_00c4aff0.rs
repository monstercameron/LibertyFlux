// original: 0x00C4AFF0 camera_update_if_active

/// Call the receiver's virtual method at table offset 0x10 when its state word
/// at offset 0x130 is zero, then return the byte flag at offset 0x2b4.
///
/// The receiver is in ECX. The call uses the receiver as ECX and takes no
/// stack arguments; its return value is ignored by this function. The contract
/// cycles the status word and the fabricated vtable target to exercise both
/// paths while recording each possible virtual target.
lf_checker_rt::export!(thiscall, rw_c4aff0(this: u32) -> u32 {
    const VTABLE_FIELD: u32 = 0;
    const METHOD_SLOT: u32 = 0x10;
    const STATE_FIELD: u32 = 0x130;
    const RESULT_FLAG: u32 = 0x2b4;

    let state = unsafe { (this.wrapping_add(STATE_FIELD) as *const u32).read_unaligned() };
    if state == 0 {
        let vtable = unsafe { (this.wrapping_add(VTABLE_FIELD) as *const u32).read_unaligned() };
        let method_address = unsafe {
            (vtable.wrapping_add(METHOD_SLOT) as *const u32).read_unaligned()
        };
        type UpdateMethod = unsafe extern "thiscall" fn(u32) -> u32;
        let method: UpdateMethod = unsafe { core::mem::transmute(method_address as usize) };
        let _ignored = unsafe { method(this) };
    }
    unsafe { (this.wrapping_add(RESULT_FLAG) as *const u8).read_unaligned() as u32 }
});
