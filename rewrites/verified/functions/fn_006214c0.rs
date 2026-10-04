// original: 0x006214c0 net_collect_passing_entries (proposed)

/// Collect the entries of a network object that pass a filter into an output
/// array, two words per slot, then order the filled slots.
///
/// `this` is a network object, `out` points to an array of 0x40-byte slots.
/// The object holds a callback pointer at `+CB_FN` with a flag word at
/// `+CB_THIS`, a filter context at `+FILTER_THIS`, a first source record at
/// `+FIRST_SRC`, a table of entry pointers at `+TABLE` and its length at
/// `+COUNT`.
///
/// When the callback is non-null it is asked first with (this, out, 0x20):
/// through the flag word as a thiscall when the flag is non-zero, otherwise
/// as a plain call. A result above 0x20 or equal to 0 falls through to the
/// main path; any other result is returned at once and nothing is written.
///
/// The main path notifies a per-slot hook (callee 2) for the first source and
/// copies its words at `+COPY_A`/`+COPY_B` into slot 0, then scans the table:
/// an entry whose first word is negative is skipped, otherwise the filter
/// (callee 3) is asked and a zero low byte accepts it, notifying the hook for
/// the entry's data (`+ENTRY_DATA`) and copying the two words into the next
/// slot. At most `MAX_SLOTS` slots are filled and the scan stops after
/// `COUNT` entries. When more than one slot was filled, a two-phase ordering
/// pass (callees 4 and 5, fastcall register pair with caller cleanup) runs
/// over (out, end) with the floor of log2(slot count) as its level, then the
/// slot count is returned.
///
/// Edge cases: null callback goes straight to the main path; a zero or
/// negative count fills slot 0 only; the ordering pass is skipped when only
/// slot 0 was filled. The original keeps its scan index and table cursor in
/// its saved-register slots, so a main-path return leaves the callee-saved
/// registers clobbered; the checker does not observe those registers.
///
/// Original: 0x006214c0 (thiscall, one stack word), returns the slot count.
lf_checker_rt::export!(thiscall, rw_006214c0(this: u32, out: u32) -> u32 {
    unsafe {
        const CB_THIS: u32 = 0x1c;
        const CB_FN: u32 = 0x20;
        const FILTER_THIS: u32 = 0x24;
        const FIRST_SRC: u32 = 0xbb8;
        const TABLE: u32 = 0x2e24;
        const COUNT: u32 = 0x2ea4;
        const COPY_A: u32 = 0x38;
        const COPY_B: u32 = 0x3c;
        const SLOT: u32 = 0x40;
        const ENTRY_DATA: u32 = 8;
        const CB_LIMIT: u32 = 0x20;
        const MAX_SLOTS: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let callback = rd32(this + CB_FN);
        if callback != 0 {
            let flag = rd32(this + CB_THIS);
            let answer = if flag != 0 {
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(callback as usize) };
                f(flag, this, out, CB_LIMIT)
            } else {
                let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(callback as usize) };
                f(this, out, CB_LIMIT)
            };
            if answer > CB_LIMIT {
                // Falls through to the main path.
            } else if answer != 0 {
                return answer;
            }
        }

        let first = this + FIRST_SRC;
        lf_checker_rt::callee_thiscall!(2, u32, out, first);
        wr32(out + COPY_A, rd32(first + COPY_A));
        wr32(out + COPY_B, rd32(first + COPY_B));

        let mut slots = 1u32;
        let count = rd32(this + COUNT) as i32;
        if count > 0 {
            let mut dst = out + SLOT;
            let mut cursor = this + TABLE;
            let mut i = 0i32;
            while i < count {
                if slots >= MAX_SLOTS {
                    break;
                }
                let entry = rd32(cursor);
                if (rd32(entry) as i32) >= 0 {
                    let filter = rd32(this + FILTER_THIS);
                    let verdict: u32 =
                        lf_checker_rt::callee_thiscall!(3, u32, filter, rd32(entry));
                    if (verdict as u8) == 0 {
                        let data = entry + ENTRY_DATA;
                        lf_checker_rt::callee_thiscall!(2, u32, dst, data);
                        wr32(dst + COPY_A, rd32(data + COPY_A));
                        wr32(dst + COPY_B, rd32(data + COPY_B));
                        dst += SLOT;
                        slots += 1;
                    }
                }
                i += 1;
                cursor += 4;
            }
        }

        if slots > 1 {
            let end = out + slots * SLOT;
            let mut level = 0u32;
            let mut rest = slots;
            if rest != 1 {
                loop {
                    rest >>= 1;
                    level += 1;
                    if rest == 1 {
                        break;
                    }
                }
            }
            lf_checker_rt::callee_fastcall!(4, u32, out, end, level, level * 2, 0u32);
            lf_checker_rt::callee_fastcall!(5, u32, out, end, 0u32);
        }
        slots
    }
});
