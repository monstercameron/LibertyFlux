// original: 0x009e15a0 audio_voice_install
/// Install a new voice/count pair, stamp the low byte onto the
/// linked record, and refresh the record's cached value through the pool.
/// Returns the pool answer, or the count when no record is linked.
/// (thiscall/2)
export!(thiscall, rw_009e15a0(this: *mut u8, voice: u32, count: u32) -> u32 {
    unsafe {
        let link = *((this.add(0x34)) as *const u32);
        *(this.add(0x30) as *mut u32) = voice;
        *(this.add(0x38) as *mut u32) = count;
        if link == 0 {
            return count;
        }
        *((link.wrapping_add(0x40)) as *mut u8) = (count & 0xFF) as u8;
        let pool = *global::<u32>(0x12FB214);
        let r = callee_thiscall!(1, u32, pool, voice);
        *((link.wrapping_add(0x48)) as *mut u32) = r;
        r
    }
});
