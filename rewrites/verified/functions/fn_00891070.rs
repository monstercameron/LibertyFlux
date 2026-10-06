// original: 0x00891070 audsound_dispatch_slot_op
/// Dispatches a slot operation through the handler table, then latches.
///
/// Returns 2 while bit 0 of the byte at `this+0x39` is set, 1 while bit 7
/// of `this+0x38` is set. Otherwise takes `arg1`, or the resolve callee's
/// (cdecl: `(int16)[this+0x3c]`) answer when `arg1` is null, as the handle;
/// calls the ready callee (thiscall, no stack words) and, when its low byte
/// is set, forces the handle to 1, skipping the dispatch; otherwise calls
/// the handler from the global table at index `[this+0x3b]` (cdecl: this,
/// handle, 1) and returns its answer at once when it is 2. Otherwise latches
/// the sample callee's (cdecl: `arg1`) low word into the resolved slot
/// (null when `[this+4]` is 0xff, else `stride * byte + table[idx * 0x6f40 +
/// 0x6f14]`): the word at +0xe4, the low-byte flag at +0xe8 retoggled from
/// `arg2`'s low two bits, bit 0x40 at +0xe7, and all-ones at +0xdc; then
/// returns the handle. (A 0xff slot byte faults identically on both sides
/// at the first slot access.)
/// Original: 0x00891070 (thiscall, two stack words: arg1, arg2).
export!(thiscall, rw_00891070(this: *mut u8, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const READY: u32 = 2;
        const DISPATCH_BASE: u32 = 0x115d654;
        const SAMPLE: u32 = 4;
        const SLOT_BYTE: usize = 4;
        const INDEX: usize = 0x40;
        const ROW: u32 = 0x6f40;
        const COL: u32 = 0x6f14;
        const EMPTY: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d968;
        const TABLE_G: u32 = 0x115d988;
        if *this.add(0x39) & 1 != 0 {
            return 2;
        }
        if *this.add(0x38) & 0x80 != 0 {
            return 1;
        }
        let mut h = if arg1 == 0 {
            let k = *(this.add(0x3c) as *const i16) as i32 as u32;
            callee_cdecl!(RESOLVE, u32, k)
        } else {
            arg1
        };
        let ready: u32 = callee_thiscall!(READY, u32, this as u32);
        if (ready as u8) == 0 {
            let idx = *this.add(0x3b) as u32;
            let target = *global::<u32>(DISPATCH_BASE.wrapping_add(idx.wrapping_mul(4)));
            let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            h = f(this as u32, h, 1);
            if h == 2 {
                return h;
            }
        } else {
            h = 1;
        }
        let s: u32 = callee_cdecl!(SAMPLE, u32, arg1);
        let di = s as u16;
        let b = *this.add(SLOT_BYTE);
        let edx = if b == EMPTY {
            0
        } else {
            let stride = *global::<u32>(STRIDE_G);
            let table = *global::<u32>(TABLE_G);
            let idx = *this.add(INDEX) as u32;
            let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL))
                as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(base)
        };
        let mut cl = (arg2 as u8).wrapping_shl(2);
        cl ^= *((edx.wrapping_add(0xe8)) as *const u8);
        ((edx.wrapping_add(0xe4)) as *mut u16).write_unaligned(di);
        cl &= 4;
        let e8 = (edx.wrapping_add(0xe8)) as *mut u8;
        *e8 ^= cl;
        *((edx.wrapping_add(0xe7)) as *mut u8) |= 0x40;
        ((edx.wrapping_add(0xdc)) as *mut u32).write_unaligned(0xffff_ffff);
        h
    }
});
