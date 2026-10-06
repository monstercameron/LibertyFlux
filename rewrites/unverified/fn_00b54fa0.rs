// original: 0x00B54FA0 crmtManagerChannel::vf7

/// Tear down channel `idx`, then slide the later channels down one slot.
///
/// Notifies the channel (callee 1), starts it (callee 2 on the channel
/// word), then fills a frame buffer twice (callees 3 and 6, each taking
/// the buffer plus (0, kind, 0, 0)) and copies the four resulting
/// doubles, as plain 8-byte moves with no arithmetic, into the slot
/// object at `slot_base`[`idx`]. After stopping the channel (callee 4)
/// every channel above `idx` slides down one slot through the mover
/// (callee 5, cdecl): slot (`p`, `p`+4) and (`p`+0x480, `p`+0x484) for
/// `p` from base+(`idx`+1)*4 while below base+32*4. The bound `idx`+1
/// (wrapping) against 32 is compared unsigned; at or above, nothing
/// slides. Two scratch words the original stamps on its frame are never
/// read and are not reproduced. Returns the last mover answer, or the
/// stop answer when nothing slid.
///
/// The buffer address differs per side and is skipped in favour of a
/// snapshot of its four words.
///
/// Original: 0x00B54FA0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54fa0(this: u32, idx: u32) -> u32 {
    unsafe {
        const CHAN_BASE: u32 = 0x30c;
        const SLOT_BASE: u32 = 0x78c;
        const COUNT: u32 = 0x20;
        const ROW_BASE: u32 = 0x308;
        const ROW_STRIDE: u32 = 0x480;
        const KIND: u32 = 0x4016a0;
        const PREP: u32 = 1;
        const START: u32 = 2;
        const FILL1: u32 = 3;
        const FILL2: u32 = 6;
        const STOP: u32 = 4;
        const MOVE: u32 = 5;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this, idx);
        let ch = (this + CHAN_BASE).wrapping_add(idx.wrapping_mul(4));
        let ch = (ch as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(START, u32, ch);
        let mut buf = [0u32; 4];
        let bp = (&mut buf as *mut u32) as u32;
        let kind = lf_checker_rt::relocated(KIND);
        let _: u32 = lf_checker_rt::callee_thiscall!(FILL1, u32, bp, 0, kind, 0, 0);
        let slot = (this + SLOT_BASE).wrapping_add(idx.wrapping_mul(4));
        let slot = (slot as *const u32).read_unaligned();
        let d0 = (bp as *const u64).read_unaligned();
        (slot as *mut u64).write_unaligned(d0);
        let d1 = ((bp + 8) as *const u64).read_unaligned();
        ((slot + 8) as *mut u64).write_unaligned(d1);
        let _: u32 = lf_checker_rt::callee_thiscall!(FILL2, u32, bp, 0, kind, 0, 0);
        let d2 = (bp as *const u64).read_unaligned();
        ((slot + 16) as *mut u64).write_unaligned(d2);
        let d3 = ((bp + 8) as *const u64).read_unaligned();
        ((slot + 24) as *mut u64).write_unaligned(d3);
        let out: u32 = lf_checker_rt::callee_thiscall!(STOP, u32, ch);
        let next = idx.wrapping_add(1);
        if next >= COUNT {
            return out;
        }
        let mut out = out;
        let mut p = (this + ROW_BASE).wrapping_add(next.wrapping_mul(4));
        let mut left = COUNT - next;
        while left != 0 {
            out = lf_checker_rt::callee_cdecl!(MOVE, u32, p, p.wrapping_add(4));
            out = lf_checker_rt::callee_cdecl!(
                MOVE, u32, p.wrapping_add(ROW_STRIDE), p.wrapping_add(ROW_STRIDE + 4)
            );
            p = p.wrapping_add(4);
            left -= 1;
        }
        out
    }
});
