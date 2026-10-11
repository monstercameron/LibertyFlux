// original: 0x00d17a10 set_task_output_fields
/// Stores the first stack argument as a word, stores the low half of the
/// second stack argument, and clears a task byte. The third and fourth stack
/// arguments are ignored; the callee releases all four words on return.

const PRIMARY_OUTPUT: u32 = 0x14B4;
const SECONDARY_OUTPUT: u32 = 0x14B8;
const OUTPUT_VALID: u32 = 0x153A;

lf_checker_rt::export!(thiscall, rw_00d17a10(
    task: u32,
    primary_value: u32,
    secondary_bits: u32,
    _unused_word_3: u32,
    _unused_word_4: u32
) -> u32 {
    unsafe {
        ((task + PRIMARY_OUTPUT) as *mut u32).write_unaligned(primary_value);
        ((task + SECONDARY_OUTPUT) as *mut u16).write_unaligned(secondary_bits as u16);
        ((task + OUTPUT_VALID) as *mut u8).write(0);
    }
    0
});
