// original: 0x009a9c70 conv_pick_min_a
/// Pick the live entry with the smallest vote count for `key`.
///
/// Asks the sub-table lookup (stubbed, stdcall/2) for the entry list
/// of `key`, with the count through a stack out-byte; a null list or
/// a zero count fails. Otherwise a random start below the count is
/// drawn (stubbed, cdecl/2 with `(0, count-1)`). Entry 0 is probed at
/// `(start) mod count`, but every later entry `i` is probed at slot
/// `i` itself: the loop reloads the divisor base with the count after
/// the first probe, discarding the start (all arithmetic signed, as
/// the original's `idiv` does it). An entry whose signed vote count
/// is not below the best so far is pruned; the rest are liveness-
/// probed (stubbed, cdecl/1), and a live entry with a lower count
/// becomes the new best. With no live entry the output word gets -1
/// and null is returned; otherwise the output word gets the best
/// count and the best id is returned. Stdcall, two stack words.
export!(stdcall, rw_009A9C70(key: u32, out: u32) -> u32 {
    unsafe {
        let mut n: u8 = 0;
        let rec: u32 = callee_stdcall!(1, u32, key, &mut n as *mut u8 as u32);
        if rec == 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let nn = n as i32;
        if nn <= 0 {
            (out as *mut u32).write_unaligned(0xFFFFFFFF);
            return 0;
        }
        let start: u32 = callee_cdecl!(2, u32, 0, (nn - 1) as u32);
        let mut best: i32 = -1;
        let mut best_slot: i32 = -1;
        let mut i = 0i32;
        while i < nn {
            // Iteration 0 probes (start) mod nn; later iterations
            // probe slot i: the start is clobbered after iter 0.
            let slot = if i == 0 {
                (start as i32).wrapping_rem(nn)
            } else {
                i
            };
            let probe_it = if best == -1 {
                true
            } else {
                let c = ((rec + (slot as u32) * 8 + 4) as *const i32).read_unaligned();
                c < best
            };
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
