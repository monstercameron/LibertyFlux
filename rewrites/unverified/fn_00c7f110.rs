// original: 0x00C7F110 CTaskComplexMobileMakeCall::create_next_sub_task

/// Query the vehicle object at receiver offset `+8` through virtual slot 3.
/// If that signed-in-register status is exactly `0x11D`, forward the explicit
/// argument through receiver virtual slot 19 and return its result; otherwise
/// return zero. The object and vtable words are 32-bit game pointers.
lf_checker_rt::export!(thiscall, rw_00c7f110(this: u32, argument: u32) -> u32 {
    const VEHICLE_OFFSET: u32 = 0x08;
    const VTABLE_POINTER_OFFSET: u32 = 0x00;
    const STATUS_SLOT_OFFSET: u32 = 0x0C;
    const MAKE_CALL_SLOT_OFFSET: u32 = 0x4C;
    const MAKING_CALL_STATUS: u32 = 0x11D;

    let vehicle = unsafe {
        (this.wrapping_add(VEHICLE_OFFSET) as *const u32).read_unaligned()
    };
    let vehicle_vtable = unsafe {
        (vehicle.wrapping_add(VTABLE_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    let status_function = unsafe {
        (vehicle_vtable.wrapping_add(STATUS_SLOT_OFFSET) as *const u32).read_unaligned()
    };
    let read_status: unsafe extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(status_function as usize)
    };
    let status = unsafe { read_status(vehicle) };
    if status == MAKING_CALL_STATUS {
        let receiver_vtable = unsafe {
            (this.wrapping_add(VTABLE_POINTER_OFFSET) as *const u32).read_unaligned()
        };
        let make_call_function = unsafe {
            (receiver_vtable.wrapping_add(MAKE_CALL_SLOT_OFFSET) as *const u32).read_unaligned()
        };
        let make_call: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(make_call_function as usize)
        };
        unsafe { make_call(this, argument) }
    } else {
        0
    }
});
