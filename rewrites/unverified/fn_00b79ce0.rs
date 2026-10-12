// original: 0x00B79CE0 CTaskComplexCarDrive::~CTaskComplexCarDrive

/// Destroy the car-drive task's timer node and vehicle reference before the base task.
///
/// `this` points to the task object. The function replaces its vtable, detaches
/// a non-null timing node stored at offset `0x40`, passes the vehicle-reference
/// slot at `0x14` to the forwarding helper, and tail-calls the base destructor.
/// The base destructor's return value is forwarded unchanged.
lf_checker_rt::export!(thiscall, rw_00b79ce0(this: u32) -> u32 {
    const TASK_VTABLE: u32 = 0x00EB33EC;
    const VEHICLE_REFERENCE: u32 = 0x14;
    const TIMING_NODE: u32 = 0x40;
    const DETACH_TIMING_NODE: u32 = 1;
    const FORWARD_VEHICLE_REFERENCE: u32 = 2;
    const BASE_DESTRUCTOR: u32 = 3;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        write_u32(this, lf_checker_rt::relocated(TASK_VTABLE));
        let timing_node = read_u32(this.wrapping_add(TIMING_NODE));
        if timing_node != 0 {
            let timing_slot = this.wrapping_add(TIMING_NODE);
            let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, timing_node, timing_slot);
        }
        let vehicle_slot = this.wrapping_add(VEHICLE_REFERENCE);
        let _ = lf_checker_rt::callee_thiscall!(FORWARD_VEHICLE_REFERENCE, u32, vehicle_slot);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
