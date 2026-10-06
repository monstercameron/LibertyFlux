// original: 0x008ea6e0 record_marker_scan
/// Clear per-entry marker bits, then repeatedly ask the worker helper for a
/// packed entry/record answer and mark the answered records.
///
/// `this` owns a table of 64 entry pointers at offset 0x804 with a parallel
/// counter array at 0xB04. Each entry holds records of 32 bytes whose last
/// byte carries flags. First every entry with a positive counter has bit 3
/// (`0x08`) cleared in the flag byte of each of its records. Then, `count`
/// times (fewer if the helper answers the 0xFFFF sentinel), the helper is
/// called with the two direction floats and the forwarded arguments; its
/// answer is stored through `out` when non-null, and bit 3 is set in the
/// answered record — unless the answer is the sentinel, which is stored
/// through `a0` and ends the scan. A final helper call runs after the loop.
/// The direction pair is normalized when `flags` is non-zero (a zero length
/// yields 1.0 for the first component and keeps the second unchanged);
/// otherwise it passes through unchanged. Always returns `a0`.
///
/// The helper returns a pointer to its answer word, which is read back
/// through that pointer; the helper also writes through its first argument,
/// which aliases the caller's count slot on loop calls.
lf_checker_rt::export!(thiscall, aq13_fn3(
    this: u32,
    a0: u32,
    a1: u32,
    a2f: f32,
    a3: u32,
    count: u32,
    a5: u32,
    out: u32,
    flags: u32,
    d0: f32,
    d1: f32,
    a10: u32,
) -> u32 {
    const TABLE_OFF: u32 = 0x804; // entry-pointer table on `this`
    const COUNT_OFF: u32 = 0xB04; // parallel per-entry counter array
    const N_ENTRIES: u32 = 0x40;
    const REC_LEN: u32 = 0x20; // record stride inside an entry
    const FLAG_OFF: u32 = 0x1F; // flag byte offset inside a record
    const MARK: u8 = 0x08; // record-marker bit
    const SENTINEL: u32 = 0xFFFF;

    /// Call the worker helper with the full 14-word argument block.
    /// Returns the helper's answer pointer, read back by the caller.
    #[inline(always)]
    fn worker(
        this: u32,
        e0: u32,
        e1: u32,
        e2f: f32,
        e3: u32,
        e5: u32,
        e8: u32,
        e9f: f32,
        e10f: f32,
        e12: u32,
    ) -> u32 {
        lf_checker_rt::callee_thiscall!(
            1, u32, this,
            e0,
            e1,
            e2f.to_bits(),
            e3,
            1,
            e5,
            0,
            0,
            e8,
            e9f.to_bits(),
            e10f.to_bits(),
            0x40400000u32,
            e12,
            0,
        )
    }

    // Direction pair: normalize when the flag byte is set, else pass through.
    // A zero length yields 1.0 for the first component (the original's exact
    // zero comparison, which a NaN length fails, taking the scaling path);
    // otherwise both scale by the reciprocal of the length. Operand order is
    // pinned to the original's scalar sequence.
    let d0 = core::hint::black_box(d0);
    let d1 = core::hint::black_box(d1);
    let mut f10 = d1;
    let mut f9 = d0;
    if (flags as u8) != 0 {
        let sq0 = core::hint::black_box(d0 * d0);
        let sq1 = core::hint::black_box(d1 * d1);
        let len_sq = core::hint::black_box(sq0 + sq1);
        let len = core::hint::black_box(len_sq.sqrt());
        if len == 0.0 {
            f9 = 1.0;
        } else {
            let s = core::hint::black_box(1.0 / len);
            f10 = core::hint::black_box(s * d1);
            f9 = core::hint::black_box(s * d0);
        }
    }

    // Clear the marker bit in every counted record of every live entry.
    let mut slot = this.wrapping_add(TABLE_OFF);
    let mut left = N_ENTRIES;
    while left != 0 {
        let entry = unsafe { (slot as *const u32).read() };
        if entry != 0 {
            let n = unsafe {
                (slot.wrapping_add(COUNT_OFF.wrapping_sub(TABLE_OFF)) as *const i32).read()
            };
            if n > 0 {
                let mut k: u32 = 1;
                while (k as i32) <= n {
                    let p = (entry as *mut u8)
                        .wrapping_add((k.wrapping_mul(REC_LEN) as usize).wrapping_sub(1));
                    unsafe {
                        let b = p.read();
                        p.write(b & !MARK);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
        slot = slot.wrapping_add(4);
        left = left.wrapping_sub(1);
    }

    // Answer loop: ask, publish, mark; the sentinel ends the scan early.
    // The helper's answer is read back through its returned pointer; the
    // first argument aliases a word holding `count` (the caller's count
    // slot on the original side), which the helper overwrites identically.
    let mut answer_slot: u32 = count;
    let mut n = count as i32;
    if n > 0 {
        while n > 0 {
            let ans_ptr = worker(
                this,
                &mut answer_slot as *mut u32 as u32,
                a1,
                a2f,
                a3,
                a5,
                flags,
                f9,
                f10,
                a10,
            );
            let ans = unsafe { (ans_ptr as *const u32).read() };
            if out != 0 {
                unsafe { (out as *mut u32).write(ans) };
            }
            if (ans & SENTINEL) == SENTINEL {
                unsafe { (a0 as *mut u32).write(ans) };
                return a0;
            }
            let entry = unsafe {
                ((this.wrapping_add(TABLE_OFF).wrapping_add((ans & SENTINEL) * 4))
                    as *const u32)
                    .read()
            };
            let rec = ans >> 16;
            let p = (entry as *mut u8)
                .wrapping_add((rec.wrapping_mul(REC_LEN).wrapping_add(FLAG_OFF)) as usize);
            unsafe {
                let b = p.read();
                p.write(b | MARK);
            }
            n = n.wrapping_sub(1);
        }
    }

    // Final call; its answer lands in the caller's word, unread here.
    worker(this, a0, a1, a2f, a3, a5, flags, f9, f10, a10);
    a0
});
