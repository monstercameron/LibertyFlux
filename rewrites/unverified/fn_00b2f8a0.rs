// original: 0x00B2F8A0 reset_three_task_slots
/// Reset four six-byte task slots through their shared helper, then initialize
/// the receiver's seven default words. The helper calls are preserved in order
/// because they may perform work beyond clearing the slot bytes. This method
/// takes its receiver in ECX, uses no stack arguments and returns the receiver.
const TASK_SLOTS_OFFSET: u32 = 0x1c;
const TASK_SLOT_STRIDE: u32 = 6;
const TASK_SLOT_COUNT: u32 = 4;

lf_checker_rt::export!(thiscall, rw_b2f8a0(this: u32) -> u32 {
    unsafe {
        for slot_index in 0..TASK_SLOT_COUNT {
            let slot = this.wrapping_add(TASK_SLOTS_OFFSET + slot_index * TASK_SLOT_STRIDE);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, slot);
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }
        write_word(this, 0);
        write_word(this.wrapping_add(4), u32::MAX);
        write_word(this.wrapping_add(8), 0);
        write_word(this.wrapping_add(12), 0);
        write_word(this.wrapping_add(16), 0);
        write_word(this.wrapping_add(20), 1000);
        write_word(this.wrapping_add(24), 2000);
        this
    }
});
