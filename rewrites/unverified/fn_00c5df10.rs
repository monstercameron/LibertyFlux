// original: 0x00C5DF10 CTaskComplexGangDriveby::vf1
/// Allocate a gang-driveby task copy and initialize it from the source task. The optional state pointer is selected by the source flag; the scalar blend field is forwarded as raw bits, and the byte mode is zero-extended.
///
/// The relocated pool global is passed to an intercepted thiscall allocator.
/// The successful path calls the task-specific initializer; the null path returns zero.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5df10(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }
        const SOURCE_TASK_OFFSET: u32 = 0x14;
        let source_task = core::ptr::read_unaligned(this.wrapping_add(SOURCE_TASK_OFFSET) as *const u32);
        const STATE_FLAG_OFFSET: u32 = 0x30;
        const OPTIONAL_STATE_OFFSET: u32 = 0x20;
        let optional_state = if (this.wrapping_add(STATE_FLAG_OFFSET) as *const u8).read() != 0 {
            this.wrapping_add(OPTIONAL_STATE_OFFSET)
        } else {
            0
        };
        const BLEND_BITS_OFFSET: u32 = 0x34;
        let blend_bits = core::ptr::read_unaligned(this.wrapping_add(BLEND_BITS_OFFSET) as *const u32);
        const STATE_WORD_A_OFFSET: u32 = 0x38;
        let state_word_a = core::ptr::read_unaligned(this.wrapping_add(STATE_WORD_A_OFFSET) as *const u32);
        const STATE_WORD_B_OFFSET: u32 = 0x3C;
        let state_word_b = core::ptr::read_unaligned(this.wrapping_add(STATE_WORD_B_OFFSET) as *const u32);
        const MODE_BYTE_OFFSET: u32 = 0x40;
        let mode_byte = (this.wrapping_add(MODE_BYTE_OFFSET) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(2, u32, allocated, source_task, optional_state, blend_bits, state_word_a, state_word_b, mode_byte)
    }
}
