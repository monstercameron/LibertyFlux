// original: 0x00B79D10 CTaskComplexCarDriveBasic::~CTaskComplexCarDriveBasic

/// Destroy the basic-drive task's timing node and animation-player reference.
///
/// `this` points to the task object. The function replaces its vtable, detaches
/// the optional timing node at offset `0x14`, and if the animation-player
/// pointer at `0x44` is non-null, restores its default state and updates its
/// blend using the exact `-4.0f32` argument. It then clears that pointer and
/// tail-calls the base destructor, forwarding its return value.
lf_checker_rt::export!(thiscall, rw_00b79d10(this: u32) -> u32 {
    const TASK_VTABLE: u32 = 0x00EB3334;
    const TIMING_NODE: u32 = 0x14;
    const ANIMATION_PLAYER: u32 = 0x44;
    const BLEND_RATE_BITS: u32 = 0xC0800000;
    const DETACH_TIMING_NODE: u32 = 1;
    const RESTORE_ANIMATION_DEFAULT: u32 = 2;
    const UPDATE_ANIMATION_BLEND: u32 = 3;
    const BASE_DESTRUCTOR: u32 = 4;

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
        let animation_player = read_u32(this.wrapping_add(ANIMATION_PLAYER));
        if animation_player != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RESTORE_ANIMATION_DEFAULT, u32, animation_player, this);
            let _ = lf_checker_rt::callee_thiscall!(UPDATE_ANIMATION_BLEND, u32, animation_player, BLEND_RATE_BITS);
            write_u32(this.wrapping_add(ANIMATION_PLAYER), 0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
