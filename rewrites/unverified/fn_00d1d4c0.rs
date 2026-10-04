// original: 0x00d1d4c0 CTaskComplexSeekCover::CTaskComplexSeekCover_2

/// Second constructor of the seek-cover composite task: installs the vtable,
/// copies the caller's position, direction triple and scalar settings into the
/// new object, initialises the two embedded sub-objects and the trailing
/// state, then either queries the cover-slot table (when the slot row for the
/// given index is populated) or marks the object as having no slot.
///
/// Arguments (thiscall, `this` in ECX, seven words on the stack):
/// `pos` is stored at `+0x30`; `dir` points at three dwords copied to
/// `+0x20..+0x28`; the low byte of `flags0` goes to `+0x5c`; bit 0 of the low
/// byte of `flags1` is xored into bit 0 of the byte at `+0x94`; `index`
/// selects a 36-byte row of the cover table and is kept at `+0x74` (`-1`
/// when the row is empty); `weight` (float bits, copied, never computed) is
/// kept at `+0x98`; the low three bits of `mode` select bits 3..5 of the word
/// at `+0x90` (bit 1 is set, bits 0 and 2 are cleared, the rest kept).
///
/// Calls, in order: the base task constructor (id 1), the two sub-object
/// initialisers at `+0x60` (id 2) and `+0x6c` (id 3), then, only when the
/// row's flag byte has any of the low three bits set, the slot query (id 4,
/// thiscall on the row pointer with `(out, 0)`, writing four words at
/// `out+0`), and finally the
/// shared tail initialiser (id 5). The query's out pointer aims four bytes
/// past the first word the constructor reads back, so the word at `+0x80`
/// always holds whatever the stack contained (the checker's defined
/// uninitialised-stack fill); the other three words come from the query. On
/// the empty-row path `+0x80..+0x88` are zero, `+0x90` keeps only bits 3..31
/// and `+0x74` is `-1`. Returns `this`.
///
/// Original: 0x00D1D4C0 (thiscall, seven stack words, returns ECX).
lf_checker_rt::export!(thiscall, rw_00d1d4c0(this: u32, pos: u32, dir: u32, flags0: u32, flags1: u32, index: u32, weight: u32, mode: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEE0A6C;
        const COVER_TABLE: u32 = 0x16FC990;
        const ROW_STRIDE: u32 = 36;
        const ROW_POPULATED_MASK: u8 = 7;
        const OFF_DIR: u32 = 0x20;
        const OFF_POS: u32 = 0x30;
        const OFF_FLAG0: u32 = 0x5c;
        const OFF_SUB0: u32 = 0x60;
        const OFF_SUB1: u32 = 0x6c;
        const OFF_INDEX: u32 = 0x74;
        const OFF_SLOT: u32 = 0x80;
        const OFF_MODE: u32 = 0x90;
        const OFF_TOG0: u32 = 0x94;
        const OFF_WEIGHT: u32 = 0x98;
        const OFF_TAIL0: u32 = 0xa0;
        const OFF_MARK: u32 = 0xb4;
        const OFF_TAIL1: u32 = 0xb8;
        const MODE_KEPT_MASK: u32 = 0xFFFF_FFC2;
        const MODE_EMPTY_MASK: u32 = 0xFFFF_FFF8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        // The original reads one word it never wrote (the query's out
        // pointer is four bytes past it), so capture our own uninitialised
        // stack word first, before any call can reuse this slot. Under the
        // contract both sides read the checker's defined fill here.
        let stack_word: u32 = core::ptr::read_volatile(
            core::mem::MaybeUninit::<u32>::uninit().as_ptr() as *const u32,
        );

        lf_checker_rt::callee_thiscall!(1, u32, this);
        // The original's immediate carries a HIGHLOW reloc: the worker maps
        // the image away from its preferred base, so the stored value is the
        // relocated address, not the file VA.
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this + OFF_DIR, rd32(dir));
        wr32(this + OFF_DIR + 4, rd32(dir + 4));
        wr32(this + OFF_DIR + 8, rd32(dir + 8));
        wr32(this + OFF_POS, pos);
        ((this + OFF_FLAG0) as *mut u8).write((flags0 & 0xFF) as u8);
        lf_checker_rt::callee_thiscall!(2, u32, this + OFF_SUB0);
        lf_checker_rt::callee_thiscall!(3, u32, this + OFF_SUB1);

        wr32(this + OFF_INDEX, index);
        wr32(this + OFF_SLOT, 0);
        wr32(this + OFF_SLOT + 4, 0);
        wr32(this + OFF_SLOT + 8, 0);
        let mut mode_bits = (rd32(this + OFF_MODE) & MODE_KEPT_MASK) | ((mode & 7) << 3);
        let toggle = (((this + OFF_TOG0) as *const u8).read() ^ ((flags1 & 0xFF) as u8)) & 1;
        mode_bits |= 2;
        ((this + OFF_TOG0) as *mut u8).write(
            ((this + OFF_TOG0) as *const u8).read() ^ toggle,
        );
        wr32(this + OFF_MODE, mode_bits);
        wr32(this + OFF_WEIGHT, weight);
        wr32(this + OFF_TAIL0, 0);
        wr32(this + OFF_TAIL0 + 4, 0);
        wr32(this + OFF_TAIL0 + 8, 0);
        wr32(this + OFF_MARK, 0xFFFF_FFFF);
        wr32(this + OFF_TAIL1, 0);
        wr32(this + OFF_TAIL1 + 4, 0);

        let row = index
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(COVER_TABLE));
        let row_flag = (row as *const u8).read();
        if row_flag & ROW_POPULATED_MASK != 0 && row != 0 {
            let mut out = [0u32; 4];
            // The query's `this` is the table row itself (ECX holds the row
            // pointer at the call); the row flag byte is its switch.
            lf_checker_rt::callee_thiscall!(4, u32, row, out.as_mut_ptr() as u32, 0);
            wr32(this + OFF_SLOT, stack_word);
            wr32(this + OFF_SLOT + 4, out[0]);
            wr32(this + OFF_SLOT + 8, out[1]);
            wr32(this + OFF_SLOT + 12, out[2]);
            let _ = out[3];
            lf_checker_rt::callee_thiscall!(5, u32, this);
        } else {
            wr32(this + OFF_SLOT + 8, 0);
            wr32(this + OFF_SLOT + 4, 0);
            wr32(this + OFF_SLOT, 0);
            wr32(this + OFF_MODE, rd32(this + OFF_MODE) & MODE_EMPTY_MASK);
            wr32(this + OFF_INDEX, 0xFFFF_FFFF);
            lf_checker_rt::callee_thiscall!(5, u32, this);
        }
        this
    }
});
