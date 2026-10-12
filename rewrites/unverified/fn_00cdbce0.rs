// original: 0x00CDBCE0 CTaskSimpleNMFlinch::vf21

/// Sets the flinch-task status when its unsigned end tick has passed and its key matches.
///
/// The method reads a zero-extended start word at `this + 0x20`, adds the
/// unsigned duration at `+0x24` with 32-bit wraparound, and compares the sum
/// with the game tick as unsigned values. If the tick is not later, the object
/// is unchanged and EAX is the end value. Otherwise the method compares the
/// key pointer stored in the image with the text at the second stack
/// argument's `+0x0c`. A match ORs bit `0x04` into the status byte at
/// `this + 0x14`; EAX is zero for a match and one for a mismatch. This is an
/// x86 thiscall method with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00cdbce0(this: u32, _unused: u32, params: u32) -> u32 {
    unsafe {
        const START_TICK: u32 = 0x20;
        const DURATION: u32 = 0x24;
        const STATUS: u32 = 0x14;
        const INPUT_TEXT: u32 = 0x0c;
        const GAME_TICK: u32 = 0x0117_35b4;
        const KEY_SLOT: u32 = 0x0105_1e00;

        let start_tick = (this.wrapping_add(START_TICK) as *const u16).read_unaligned() as u32;
        let duration = (this.wrapping_add(DURATION) as *const u32).read_unaligned();
        let end_tick = start_tick.wrapping_add(duration);
        let current_tick = lf_checker_rt::global::<u32>(GAME_TICK).read();
        if current_tick <= end_tick {
            return end_tick;
        }

        let key = lf_checker_rt::global::<u32>(KEY_SLOT).read_unaligned();
        let text = params.wrapping_add(INPUT_TEXT);
        let mut byte_offset = 0u32;
        let matched = loop {
            let key_byte = (key.wrapping_add(byte_offset) as *const u8).read();
            let text_byte = (text.wrapping_add(byte_offset) as *const u8).read();
            if key_byte != text_byte {
                break false;
            }
            if key_byte == 0 {
                break true;
            }
            byte_offset = byte_offset.wrapping_add(1);
        };

        if matched {
            let status = this.wrapping_add(STATUS) as *mut u8;
            status.write(status.read() | 4);
            0
        } else {
            1
        }
    }
});
