// original: 0x008a94f0 audio_table_reset
/// Reset the voice table: fill 1000 entries with -1 and reinit the tail.
///
/// Writes -1 into the 1000 dwords at `this` (the counter runs 0x3e7 down
/// through 0, inclusive on both ends), clears the header at `this+0x28a0`,
/// runs the tail reinitialiser (thiscall/0 on `this+0x3210`, stubbed), then
/// clears the two tail words and returns `this`.
export!(thiscall, rw_008a94f0(this: *mut u8) -> u32 {
    unsafe {
        const ENTRIES: u32 = 0x3E8;
        let words = this as *mut u32;
        for i in 0..ENTRIES {
            *words.add(i as usize) = 0xFFFF_FFFF;
        }
        *(this.add(0x28a0) as *mut u32) = 0;
        *(this.add(0x28a4) as *mut u16) = 0;
        callee_thiscall!(1, u32, this.add(0x3210) as u32);
        *(this.add(0x3230) as *mut u16) = 0;
        *(this.add(0x3208) as *mut u32) = 0;
        this as u32
    }
});

/// Honesty mutant of `rw_008a94f0`: fills one entry too few.
export!(thiscall, mut_008a94f0(this: *mut u8) -> u32 {
    unsafe {
        let words = this as *mut u32;
        for i in 0..0x3E7u32 {
            // BUG: should be 0x3E8
            *words.add(i as usize) = 0xFFFF_FFFF;
        }
        *(this.add(0x28a0) as *mut u32) = 0;
        *(this.add(0x28a4) as *mut u16) = 0;
        callee_thiscall!(1, u32, this.add(0x3210) as u32);
        *(this.add(0x3230) as *mut u16) = 0;
        *(this.add(0x3208) as *mut u32) = 0;
        this as u32
    }
});
