// original: 0x009a9040 slot_table_sweep
/// Sweep the 100-entry slot table, retiring entries whose owner is gone.
///
/// Each entry is 16 bytes starting at `this+0x33d4`: an owner word at
/// `+0x00`, a state word at `+0x04` and a flag byte at `+0x08`. An entry
/// whose state is non-negative, whose flag byte is clear and whose owner
/// is zero is retired by clearing the flag and setting the state to -1.
/// Thiscall, one ignored stack word (the callee pops 4 bytes), no result.
export!(thiscall, rw_009A9040(this: u32, _unused: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x33d4;
        const COUNT: u32 = 100;
        const STRIDE: u32 = 16;
        let mut i = 0u32;
        while i < COUNT {
            let e = this + TABLE + i * STRIDE;
            let state = (e + 4) as *const i32;
            if state.read_unaligned() >= 0
                && ((e + 8) as *const u8).read() == 0
                && (e as *const u32).read_unaligned() == 0
            {
                ((e + 8) as *mut u8).write(0);
                (state as *mut i32).write_unaligned(-1);
            }
            i += 1;
        }
        0
    }
});
