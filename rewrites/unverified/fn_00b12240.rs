// original: 0x00B12240 CHeli::vf8

/// Clear the model reference and related fields. The manager lookup is made
/// only when the signed model index is greater than -1; a non-null lookup
/// result receives this object. The method resets the model index to -1,
/// clears the resource fields and zeroes the byte at offset 0x40.
lf_checker_rt::export!(thiscall, rw_00b12240(this: u32) -> () {
    const MODEL_INDEX: u32 = 0x48;
    const RESOURCE: u32 = 0xB0;
    const RESOURCE_COUNT: u32 = 0xB8;
    const ENABLED: u32 = 0x40;
    const MANAGER_VA: u32 = 0x012F_B214;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let index = read_u32(this.wrapping_add(MODEL_INDEX));
        if (index as i32) > -1 {
            let manager = read_u32(lf_checker_rt::global::<u32>(MANAGER_VA) as u32);
            let model = lf_checker_rt::callee_thiscall!(1, u32, manager, index);
            if model != 0 {
                let _ = lf_checker_rt::callee_thiscall!(2, u32, model, this);
            }
        }
        write_u32(this.wrapping_add(RESOURCE), 0);
        write_u32(this.wrapping_add(RESOURCE_COUNT), 0);
        write_u32(this.wrapping_add(MODEL_INDEX), u32::MAX);
        (this.wrapping_add(ENABLED) as *mut u8).write(0);
    }
});
