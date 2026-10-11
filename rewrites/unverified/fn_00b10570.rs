// original: 0x00B10570 CHeli::vf7

/// Resolve the helicopter's signed model index when it is nonnegative, store
/// the resolved pointer in the shared result slot, and notify that model with
/// this object and the signed 16-bit subtype at offset 0x44. The signed
/// 32-bit model index at offset 0x48 is skipped when it is below zero. The
/// model manager and result slot are relocated globals; both outgoing methods
/// are intercepted by the checker. This is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00b10570(this: u32) -> u32 {
    const MODEL_INDEX: u32 = 0x48;
    const SUBTYPE: u32 = 0x44;
    const MODEL_MANAGER_VA: u32 = 0x012F_B214;
    const RESULT_SLOT_VA: u32 = 0x0163_2C10;

    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let index = read_u32(this.wrapping_add(MODEL_INDEX));
        if (index as i32) < 0 {
            return 0;
        }
        let manager = read_u32(lf_checker_rt::global::<u32>(MODEL_MANAGER_VA) as u32);
        let model = lf_checker_rt::callee_thiscall!(1, u32, manager, index);
        write_u32(lf_checker_rt::global::<u32>(RESULT_SLOT_VA) as u32, model);
        if model != 0 {
            let subtype = read_u16(this.wrapping_add(SUBTYPE)) as i16 as i32 as u32;
            let _ = lf_checker_rt::callee_thiscall!(2, u32, model, this, subtype);
        }
        0
    }
});
