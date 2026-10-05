// original: 0x008F8DB0 stream_mgr_init_buffers
/// Initialize the streaming manager's buffers and both lanes.
///
/// Zeroes the two 0x258-word tables, clears the three header words,
/// sets the ready word, picks one of two global rate floats by a
/// probe call, writes the header constants, then initializes both
/// lanes (0x4b8 bytes apart) through the slot routine and marks each
/// lane's status words. Thiscall, no stack arguments; returns the
/// second lane's slot pointer.
export!(thiscall, rw_008f8db0(this: u32) -> u32 {
    unsafe {
        const WORDS: u32 = 0x258;
        const TABLE_B: u32 = 0x4b0;
        const RATE_A: u32 = 0x1033120;
        const RATE_B: u32 = 0x1033124;
        const LANE_STRIDE: u32 = 0x4b8;
        for i in 0..WORDS {
            ((this + i * 2) as *mut u16).write_unaligned(0);
            ((this + i * 2 + TABLE_B) as *mut u16).write_unaligned(0);
        }
        ((this + 0x9a8) as *mut u32).write_unaligned(0);
        ((this + 0x9ac) as *mut u32).write_unaligned(0);
        ((this + 0x9b0) as *mut u32).write_unaligned(0);
        ((this + 0x9b4) as *mut u32).write_unaligned(1);
        let t: u32 = callee_cdecl!(1, u32,);
        let f = if (t as u8) != 0 {
            *global::<u32>(RATE_A)
        } else {
            *global::<u32>(RATE_B)
        };
        ((this + 0x9b8) as *mut u32).write_unaligned(f);
        ((this + 0x9bc) as *mut u32).write_unaligned(0x01000000);
        ((this + 0x9c0) as *mut u32).write_unaligned(0xff834293);
        let mut e = this.wrapping_add(0x9c6);
        let mut bp = this.wrapping_add(0x9a0);
        let mut bx = this.wrapping_add(0x980);
        for di in 0..2u32 {
            callee_thiscall!(2, u32, e);
            ((bx.wrapping_sub(0x20)) as *mut u8).write(0);
            ((bx) as *mut u8).write(0);
            ((bp) as *mut u32).write_unaligned(0xFFFFFFFF);
            ((this + di + 0x9c4) as *mut u8).write(0);
            e = e.wrapping_add(LANE_STRIDE);
            bx = bx.wrapping_add(0x10);
            bp = bp.wrapping_add(4);
        }
        e
    }
});
