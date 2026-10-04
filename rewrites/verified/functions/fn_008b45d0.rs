// original: 0x008b45d0 four_way_count_table_dispatch
use lf_checker_rt::{callee_cdecl, export, global};

/// Four-way dispatch of one identifier through a shared count and table.
///
/// The entry call resolves the argument (tag 0x1a) to an index, or to the
/// NONE sentinel to take nothing further. A probe pair then selects one of
/// four arms; anything outside the four exits. Arms 0 and 1 re-resolve the
/// argument (tag 0x1c), require the shared count to exceed two, and branch
/// on the indexed table word: a set word reports directly, a clear word
/// notifies first (with an arm-specific flag) and then reports. Arm 2
/// requires the count to exceed one, notifies with the index, and reports.
/// Arm 3 notifies with the index, re-resolves, notifies again unless the
/// second resolution is NONE, and reports twice. The value left in EAX is
/// whatever the last executed step produced.
export!(cdecl, rw_008b45d0(arg: u32) -> u32 {
    /// Shared count source, passed by value to the counter and reporter.
    const COUNT_SRC: u32 = 0x0116_0C20;
    /// Base of the table indexed by the resolved index.
    const TABLE: u32 = 0x0116_0C48;
    /// Sentinel meaning "no resolution".
    const NONE: u32 = 0x7FFF_FFFF;
    unsafe {
        let shared = global::<u32>(COUNT_SRC).read();
        let index = callee_cdecl!(1, u32, arg, 0x1a);
        if index == NONE {
            return index;
        }
        let probe = callee_cdecl!(2, u32, arg);
        let which = callee_cdecl!(3, u32, probe).wrapping_sub(1);
        if which > 3 {
            return which;
        }
        if which <= 1 {
            let slot = callee_cdecl!(1, u32, arg, 0x1c);
            if slot == NONE {
                return slot;
            }
            let count = callee_cdecl!(4, u32, shared);
            if (count as i32) <= 2 {
                return count;
            }
            let cell =
                (global::<u8>(TABLE).wrapping_add(index.wrapping_mul(4) as usize)
                    as *const u32)
                    .read();
            if cell != 0 {
                return callee_cdecl!(6, u32, shared, 2, 1);
            }
            if which == 0 {
                callee_cdecl!(5, u32, slot, 0);
            } else {
                callee_cdecl!(5, u32, slot, 2);
            }
            return callee_cdecl!(6, u32, shared, 2, 0);
        }
        if which == 2 {
            let count = callee_cdecl!(4, u32, shared);
            if (count as i32) <= 1 {
                return count;
            }
            callee_cdecl!(5, u32, index, 1);
            return callee_cdecl!(6, u32, shared, 1, 0);
        }
        let count = callee_cdecl!(4, u32, shared);
        if (count as i32) <= 2 {
            return count;
        }
        callee_cdecl!(5, u32, index, 0);
        let slot = callee_cdecl!(1, u32, arg, 0x1c);
        if slot != NONE {
            callee_cdecl!(5, u32, slot, 0);
        }
        callee_cdecl!(6, u32, shared, 1, 0);
        callee_cdecl!(6, u32, shared, 2, 0)
    }
});
