// original: 0x009BB3F0 camera_script_ref_release (proposed)

/// Release one script-camera reference and update its active byte.
///
/// The 8-bit reference count at `this + 0x530` is decremented with wrapping
/// arithmetic and written through the stack pointer. If the resulting byte,
/// interpreted as signed i8, is not positive, the active byte at `+0x531` is
/// cleared. EAX returns the output pointer.
lf_checker_rt::export!(thiscall, rw_009bb3f0(this: u32, count_out: u32) -> u32 {
    unsafe {
        const REFCOUNT: u32 = 0x530;
        const ACTIVE: u32 = 0x531;
        let count_slot = this.wrapping_add(REFCOUNT) as *mut u8;
        let count = count_slot.read().wrapping_sub(1);
        count_slot.write(count);
        if (count as i8) <= 0 {
            (this.wrapping_add(ACTIVE) as *mut u8).write(0);
        }
        (count_out as *mut u8).write(count);
        count_out
    }
});
