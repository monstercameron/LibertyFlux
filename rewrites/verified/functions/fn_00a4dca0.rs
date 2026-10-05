// original: 0x00A4DCA0 NativeImpl_SET_CAR_ENGINE_ON

/// Sets engine state from the argument, or derives it through two measuring
/// callees and reports it through a third.
///
/// With a non-zero argument byte: forces the state byte at `this + STATE`
/// (0x0F14) to `(state & 0xEF) | 0x48` and returns. With a zero argument and
/// bits 3-4 already set, returns without touching anything. Otherwise sets
/// bit 4; when the linked object at `[this + LINK]` (0x6C) is present and its
/// byte at `+0x0E` is non-zero, also sets bit 5. When the link is absent or
/// unflagged, calls the measuring callee (`this` in `ecx`, float answer),
/// feeds its bits to the classifying callee (one word, low byte used) and
/// sets bit 5 of the state to the answer's low bit. Either way, pushes
/// `bit5(state) == 0` to the reporting callee (`this + REPORT`, 0x210, in
/// `ecx`), snapshots the global at `RATE` into `this + SNAP` (0x0F38), and
/// returns nothing defined.
///
/// Original: 0x00A4DCA0 (thiscall, one byte-ish stack word), three callees.
lf_checker_rt::export!(thiscall, rw_00A4DCA0(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x0F14;
        const LINK: u32 = 0x6C;
        const LINK_FLAG: u32 = 0x0E;
        const REPORT: u32 = 0x210;
        const SNAP: u32 = 0x0F38;
        const RATE: u32 = 0x011735B4;
        const FORCE_MASK: u8 = 0xEF;
        const FORCE_BITS: u8 = 0x48;
        const BUSY_BITS: u8 = 0x18;
        const BIT4: u8 = 0x10;
        const BIT5: u8 = 0x20;
        const MEASURE_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const REPORT_CALLEE: u32 = 3;
        let state = (this + STATE) as *mut u8;
        if (arg & 0xFF) != 0 {
            state.write(state.read() & FORCE_MASK | FORCE_BITS);
            return 0;
        }
        if state.read() & BUSY_BITS != 0 {
            return 0;
        }
        let link = ((this + LINK) as *const u32).read_unaligned();
        state.write(state.read() | BIT4);
        let linked =
            link != 0 && ((link + LINK_FLAG) as *const u8).read() != 0;
        if linked {
            state.write(state.read() | BIT5);
        } else {
            let r: f32 =
                lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, f32, this);
            let ans: u32 = lf_checker_rt::callee_cdecl!(
                CLASSIFY_CALLEE,
                u32,
                r.to_bits()
            );
            let s = state.read() & !BIT5 | (((ans & 1) << 5) as u8);
            state.write(s);
        }
        let off = u32::from(state.read() >> 5 & 1 ^ 1);
        lf_checker_rt::callee_thiscall!(REPORT_CALLEE, u32, this + REPORT, off);
        let g = lf_checker_rt::global::<u32>(RATE).read_unaligned();
        ((this + SNAP) as *mut u32).write_unaligned(g);
        0
    }
});
