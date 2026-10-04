// original: 0x00898bc0 audio_round_robin_advance
/// Advances a round-robin selector and notifies the audio pool.
///
/// Steps the word at offset 0x60 forward by one modulo 3 and tells the pool
/// the previous and next positions, storing the next position back and
/// returning the pool's answer. When no pool is registered nothing is stored
/// and the result is 0.
export!(thiscall, rw_00898bc0(this: u32) -> u32 {
    unsafe {
        let cur = core::ptr::read_unaligned(this.wrapping_add(0x60) as *const u16) as u32;
        let next = (cur + 1) % 3;
        let pool = core::ptr::read_unaligned(global::<u32>(0x115F810));
        if pool == 0 {
            return 0;
        }
        let ans = callee_thiscall!(1, u32, pool, cur, next);
        core::ptr::write_unaligned(this.wrapping_add(0x60) as *mut u16, next as u16);
        ans
    }
});
