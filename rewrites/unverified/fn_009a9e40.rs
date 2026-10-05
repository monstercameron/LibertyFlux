// original: 0x009a9e40 conv_pick_min_b
/// Pick the live entry with the smallest count for `key` (two levels).
///
/// Asks the conversation hub (stubbed, thiscall/1) for the record of
/// `key`. The count byte at record `+0x0a` selects a sub-record at
/// `record + count*8 + 0x0b`, whose byte is the entry count; a null
/// record or a zero count fails. Otherwise a random start is drawn
/// (stubbed, cdecl/2), iteration `i` probes `(i + start) mod count`
/// (signed arithmetic, the start reloaded after every probe). Entries are
/// 8-byte id/count pairs; an entry whose signed count is not below
/// the best is pruned, the rest are liveness-probed (stubbed,
/// cdecl/1). With no live entry the output word gets -1 and null is
/// returned; otherwise the output word gets the best count and the
/// best id is returned. Stdcall, two stack words.
export!(stdcall, rw_009A9E40(key: u32, out: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        let rec0: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec0 == 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let n1 = ((rec0 + 0x0a) as *const u8).read() as u32;
        let sub = rec0 + n1 * 8 + 0x0b;
        let nn = (sub as *const u8).read() as i32;
        if nn <= 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let rec = sub.wrapping_add(1);
        let start: u32 = callee_cdecl!(2, u32, 0, (nn - 1) as u32);
        let mut best: i32 = -1;
        let mut best_slot: i32 = -1;
        let mut i = 0i32;
        while i < nn {
            let slot = (i.wrapping_add(start as i32)).wrapping_rem(nn);
            let mut probe_it = best == -1;
            if !probe_it {
                let c = ((rec + (slot as u32) * 8 + 4) as *const i32).read_unaligned();
                probe_it = c < best;
            }
            if probe_it {
                let id = ((rec + (slot as u32) * 8) as *const u32).read_unaligned();
                let live: u32 = callee_cdecl!(3, u32, id);
                if live as u8 != 0 {
                    best = ((rec + (slot as u32) * 8 + 4) as *const i32).read_unaligned();
                    best_slot = slot;
                }
            }
            i += 1;
        }
        if best_slot < 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let e = rec + (best_slot as u32) * 8;
        (out as *mut u32).write_unaligned((e.wrapping_add(4) as *const u32).read_unaligned());
        (e as *const u32).read_unaligned()
    }
});
