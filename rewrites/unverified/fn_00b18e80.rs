// original: 0x00b18e80 sweep_tagged_slots (proposed)

/// Sweeps the slot array from the top, retiring tagged live slots.
///
/// Cdecl of one stack word: a retire budget. The slot array header gives
/// a base pointer, a flag table, a count and a stride. Indices run from
/// count-1 down to 0 while the budget lasts: an index is skipped when
/// its flag byte has 0x80 set, when its slot address is null, when the
/// slot's kind byte is not 3, or when the liveness probe (thiscall, no
/// stack arguments) reports nonzero. Otherwise the slot is retired
/// (cdecl of the slot pointer) and the budget drops by one. Returns the
/// last probe or retire result, or the flag table pointer. The empty
/// array and non-positive budget entries are excluded from the proof:
/// they return the caller's EAX, which no rewrite can observe.
lf_checker_rt::export!(cdecl, rw_00b18e80(budget: u32) -> u32 {
    unsafe {
        const HEADER_PTR: u32 = 0x01632c60;
        const KIND_TAG: u8 = 3;
        const SKIP_BIT: u8 = 0x80;
        const KIND_OFF: u32 = 0x22a;
        let header =
            (lf_checker_rt::global::<u32>(HEADER_PTR) as *const u32).read_unaligned();
        let mut index = ((header + 8) as *const u32).read_unaligned();
        let mut left = budget as i32;
        let mut answer = 0u32;
        loop {
            index = index.wrapping_sub(1);
            if left <= 0 {
                break;
            }
            let flags = ((header + 4) as *const u32).read_unaligned();
            answer = flags;
            if ((flags.wrapping_add(index)) as *const u8).read() & SKIP_BIT != 0 {
                if index == 0 {
                    break;
                }
                continue;
            }
            let stride = ((header + 0x0c) as *const u32).read_unaligned();
            let base = (header as *const u32).read_unaligned();
            let slot = base.wrapping_add(stride.wrapping_mul(index));
            if slot == 0 {
                if index == 0 {
                    break;
                }
                continue;
            }
            if ((slot + KIND_OFF) as *const u8).read() != KIND_TAG {
                if index == 0 {
                    break;
                }
                continue;
            }
            let live: u32 = lf_checker_rt::callee_thiscall!(1, u32, slot);
            answer = live;
            if (live as u8) != 0 {
                if index == 0 {
                    break;
                }
                continue;
            }
            answer = lf_checker_rt::callee_cdecl!(2, u32, slot);
            left -= 1;
            if index == 0 {
                break;
            }
        }
        answer
    }
});
