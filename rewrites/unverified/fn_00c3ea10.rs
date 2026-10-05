// original: 0x00c3ea10 train_init_carriage_slots (proposed)
/// Initialise the twelve carriage records and attach a fresh sub-object.
///
/// Writes thirteen 0x30-byte records starting at `this`. Each record
/// holds: dwords 0, 0, 1.0f, 0, 0, 1000, -1, -1, -1, 1 at word offsets
/// 0..9 and a zero byte at `+0x2c` (bytes `+0x28..+0x2b` and `+0x2d..+0x2f`
/// keep their old values). Then calls allocator id 1 (cdecl, 0x80 bytes);
/// when it returns non-null, constructor id 2 runs on the block (thiscall)
/// and its answer is stored at `this+0x274`, with `this+0x278` cleared,
/// else both slots are cleared. Returns this.
///
/// Original: 0x00c3ea10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3ea10(this: u32) -> u32 {
    unsafe {
        const FIRST_REC: u32 = 0x00;
        const STRIDE: u32 = 0x30;
        // Thirteen: the counter runs 11..0 and then -1, storing each
        // time because the sign check sits after the stores.
        const COUNT: u32 = 13;
        const UNIT: u32 = 0x3f80_0000; // 1.0f
        const CODE: u32 = 1000;
        const ALLOC_BYTES: u32 = 0x80;
        const SLOT: u32 = 0x274;
        const SLOT2: u32 = 0x278;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        #[inline(always)]
        unsafe fn w32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        let mut rec = this.wrapping_add(FIRST_REC);
        let mut i = 0u32;
        while i < COUNT {
            w32(rec, 0);
            w32(rec + 4, 0);
            w32(rec + 8, UNIT);
            w32(rec + 12, 0);
            w32(rec + 16, 0);
            w32(rec + 20, CODE);
            w32(rec + 24, 0xffff_ffff);
            w32(rec + 28, 0xffff_ffff);
            w32(rec + 32, 0xffff_ffff);
            w32(rec + 36, 1);
            ((rec + 44) as *mut u8).write(0);
            rec = rec.wrapping_add(STRIDE);
            i += 1;
        }
        let blk: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, ALLOC_BYTES);
        if blk == 0 {
            w32(this + SLOT, 0);
            w32(this + SLOT2, 0);
        } else {
            let built: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, blk);
            w32(this + SLOT, built);
            w32(this + SLOT2, 0);
        }
        this
    }
});
