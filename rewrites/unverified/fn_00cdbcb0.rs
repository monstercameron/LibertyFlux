// original: 0x00CDBCB0 CTaskSimpleNMBrace::vf21

/// Requests a brace-task blend-out after its unsigned end tick.
///
/// The method reads a zero-extended start word at `this + 0x20`, adds the
/// unsigned duration at `+0x24` with 32-bit wraparound, and compares that sum
/// with the game tick as unsigned values. If the tick is later, it sets the
/// status byte at `+0x14`, rewrites the blend word at `+0x1c`, and rewrites
/// task flags at `+0xc0`. If the tick has not passed the end value, it leaves
/// the object unchanged. EAX is the end value on the skipped path and the new
/// task-flags word on the blend-out path. This is an x86 thiscall method with
/// no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cdbcb0(this: u32) -> u32 {
    unsafe {
        const START_TICK: u32 = 0x20;
        const DURATION: u32 = 0x24;
        const STATUS: u32 = 0x14;
        const BLEND_WORD: u32 = 0x1c;
        const TASK_FLAGS: u32 = 0xc0;
        const GAME_TICK: u32 = 0x0117_35b4;
        const BLEND_MASK: u32 = 0xff08_45ff;
        const BLEND_BITS: u32 = 0x0008_4500;
        const FLAG_MASK: u32 = 0xfff2_ffff;
        const BLEND_FLAG: u32 = 0x0002_0000;

        let start_tick = (this.wrapping_add(START_TICK) as *const u16).read_unaligned() as u32;
        let duration = (this.wrapping_add(DURATION) as *const u32).read_unaligned();
        let end_tick = start_tick.wrapping_add(duration);
        let current_tick = lf_checker_rt::global::<u32>(GAME_TICK).read();
        if current_tick <= end_tick {
            return end_tick;
        }

        let status = this.wrapping_add(STATUS) as *mut u8;
        status.write(status.read() | 4);

        let blend_word = this.wrapping_add(BLEND_WORD) as *mut u32;
        let previous_blend = blend_word.read_unaligned();
        blend_word.write_unaligned((previous_blend & BLEND_MASK) | BLEND_BITS);
        (this.wrapping_add(BLEND_WORD) as *mut u8).write(4);

        let task_flags = this.wrapping_add(TASK_FLAGS) as *mut u32;
        let updated_flags = (task_flags.read_unaligned() & FLAG_MASK) | BLEND_FLAG;
        task_flags.write_unaligned(updated_flags);
        updated_flags
    }
});
