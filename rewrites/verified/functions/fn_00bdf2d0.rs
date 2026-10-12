// original: 0x00BDF2D0 CTaskSimpleSayAudio::CTaskSimpleSayAudio

/// Initializes the audio task, optionally attaches its resource pointer, copies
/// three float arguments by their original 32-bit representation, and copies a
/// NUL-terminated name into the inline buffer at offset 0x1c. The source name
/// must be readable through its terminator, as the original has no null or
/// length guard. Thiscall returns the receiver and pops five argument slots.
lf_checker_rt::export!(thiscall, rw_bdf2d0(
    this: u32,
    name: u32,
    float_a_bits: u32,
    float_b_bits: u32,
    resource: u32,
    float_c_bits: u32,
) -> u32 {
    unsafe {
        const BASE_TASK_CTOR: u32 = 1;
        const ATTACH_FREE_NODE: u32 = 2;
        const AUDIO_TASK_VTABLE: u32 = 0x00EB8B8C;
        const NODE_POINTER: u32 = 0x44;
        const NAME_FIELD: u32 = 0x1c;
        const FLOAT_A_FIELD: u32 = 0x3c;
        const FLOAT_B_FIELD: u32 = 0x40;
        const FLOAT_C_FIELD: u32 = 0x48;

        let _ = lf_checker_rt::callee_thiscall!(BASE_TASK_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(AUDIO_TASK_VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FLOAT_A_FIELD) as *mut u32).write_unaligned(float_a_bits);
        (this.wrapping_add(FLOAT_B_FIELD) as *mut u32).write_unaligned(float_b_bits);
        (this.wrapping_add(FLOAT_C_FIELD) as *mut u32).write_unaligned(float_c_bits);

        let node_pointer = this.wrapping_add(NODE_POINTER);
        (node_pointer as *mut u32).write_unaligned(resource);
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_FREE_NODE, u32, resource, node_pointer);
        }

        let mut index = 0u32;
        loop {
            let byte = (name.wrapping_add(index) as *const u8).read();
            (this.wrapping_add(NAME_FIELD).wrapping_add(index) as *mut u8).write(byte);
            index = index.wrapping_add(1);
            if byte == 0 {
                break;
            }
        }
        this
    }
});
