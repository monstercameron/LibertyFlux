// original: 0x00d31100 CTaskComplexNewExitVehicle::CTaskComplexNewExitVehicle
/// Constructs a new-exit-vehicle task by delegating base initialization, storing the vehicle, raw
/// speed bits and handle, initializing the embedded animation object, and setting default state
/// fields. A nonpositive ordered speed triggers one helper call; an unordered speed skips it. A
/// non-null vehicle is checked through a signed 16-bit model index and a global model table
/// before setting a flag; a non-null handle is then retained through its field address. The
/// thiscall method receives four stack words and returns this.
lf_checker_rt::export!(thiscall, rw_00d31100(this: u32, vehicle: u32, task_code: u32, speed_bits: u32, handle: u32) -> u32 {
    const BASE_TASK: u32 = 0x14;
    const VEHICLE_INDEX: u32 = 0x2e;
    const MODEL_TABLE_VA: u32 = 0x01295cd8;
    const MODEL_SPECIAL_WORD: u32 = 0xc4;
    const MODEL_SPECIAL_VALUE: u32 = 0x57;
    const TASK_FLAGS: u32 = 0x14;
    const VTABLE_VA: u32 = 0x00ee31dc;
    const SPEED_BITS: u32 = 0x44;
    const HANDLE_SLOT: u32 = 0x48;
    const ANIMATION_DATA: u32 = 0x60;
    const ANIMATION_STATUS: u32 = 0x86;
    const ANIMATION_FLAGS: u32 = 0x8b;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn read_u16(base: u32, offset: u32) -> u16 {
        unsafe { (base.wrapping_add(offset) as *const u16).read_unaligned() }
    }
    unsafe fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, vehicle, u32::MAX, task_code, u32::MAX);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, 0);
        write_u32(this, SPEED_BITS, speed_bits);
        write_u32(this, HANDLE_SLOT, handle);
        write_u32(this, 0x50, 0);
        write_u32(this, 0x54, 0);
        write_u32(this, 0x58, 0);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(ANIMATION_DATA));
        write_u32(this, 0x38, 0);
        write_u32(this, 0x3c, 0);
        write_u32(this, 0x40, 0);
        if f32::from_bits(speed_bits) <= 0.0f32 {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, this, 1);
        }
        if vehicle != 0 {
            let model_index = read_u16(vehicle, VEHICLE_INDEX) as i16;
            let table = lf_checker_rt::global::<u32>(MODEL_TABLE_VA);
            let model_data = table.wrapping_offset(model_index as isize).read_unaligned();
            if read_u32(model_data, MODEL_SPECIAL_WORD) == MODEL_SPECIAL_VALUE {
                let old_flags = read_u32(this, TASK_FLAGS);
                write_u32(this, TASK_FLAGS, old_flags | 0x100);
            }
        }
        if handle != 0 {
            let slot = this.wrapping_add(HANDLE_SLOT);
            let _ = lf_checker_rt::callee_thiscall!(4, u32, handle, slot);
        }
        let animation_flags = read_u8(this, ANIMATION_FLAGS);
        write_u8(this, ANIMATION_FLAGS, animation_flags | 0x80);
        write_u8(this, ANIMATION_STATUS, 5);
        this
    }
});
