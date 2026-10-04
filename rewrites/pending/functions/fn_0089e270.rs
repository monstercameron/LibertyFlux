// original: 0x0089E270 rage::audSequentialSound::~audSequentialSound__deleting
// ---------------------------------------------------------------------------
// 0x0089E270 rage::audSequentialSound deleting destructor: run the
// sequential-sound teardown, then when the caller's dispose flag (bit 0) is
// set release this object back to the audio pool keyed by its bank byte.
// Returns this.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089E270(this_ptr: u32, flags: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr);
    if flags & 1 != 0 && this_ptr != 0 {
        let bank = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        callee_thiscall!(2, u32, relocated(0x115D8A0), this_ptr, bank);
    }
    this_ptr
});
