// original: 0x00CB0010 copy_task_position_and_speed

/// Copy a three-word vector from the second stack argument to the output pointer in the first, store the third stack argument as this object's speed at offset 0x18, set flag bit 0x4 at offset 0x54, and return the output pointer. The thiscall method pops three stack words.
lf_checker_rt::export!(thiscall, rw_00cb0010(this: u32, output: u32, source: u32, speed: f32) -> u32 {
unsafe {
        const SPEED: u32 = 0x18;
        const FLAGS: u32 = 0x54;
        let flag_word = (this + FLAGS) as *mut u32;
        flag_word.write_unaligned(flag_word.read_unaligned() | 0x4);
        ((this + SPEED) as *mut f32).write_unaligned(speed);
        for offset in [0u32, 4, 8] {
            let word = ((source + offset) as *const u32).read_unaligned();
            ((output + offset) as *mut u32).write_unaligned(word);
        }
        output
    }
});
