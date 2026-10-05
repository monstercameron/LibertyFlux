// original: 0x009a9f20 conv_pick_min_c
/// Pick the live entry with the smallest count for `key` (one level).
///
/// Asks the conversation hub (stubbed, thiscall/1) for the record of
/// `key`; a null record or a zero count byte at record `+0x0a` fails.
/// Otherwise a random start is drawn (stubbed, cdecl/2), iteration 0
/// probes `(start) mod count` and later iterations probe slot `i`
/// (signed arithmetic). Entries pack id and count unaligned at
/// `record + 0x0b + slot*8` (id) and `+4` past that (count); an entry
/// whose signed count is not below the best is pruned, the rest are
/// liveness-probed (stubbed, cdecl/1). With no live entry the output
/// word gets -1 and null is returned; otherwise the output word gets
/// the best count and the best id is returned. Stdcall, two words.
export!(stdcall, rw_009A9F20(key: u32, out: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        let rec0: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec0 == 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let nn = ((rec0 + 0x0a) as *const u8).read() as i32;
        if nn <= 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let rec = rec0.wrapping_add(0x0b);
        let start: u32 = callee_cdecl!(2, u32, 0, (nn - 1) as u32);
        let mut best: i32 = -1;
        let mut best_slot: i32 = -1;
        let mut i = 0i32;
        while i < nn {
            let slot = if i == 0 {
                (start as i32).wrapping_rem(nn)
            } else {
                i
            };
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
