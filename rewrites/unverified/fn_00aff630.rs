// original: 0x00AFF630 pool_cull_sort
/// Filter one vehicle pool pass using the mode byte, live-slot bitmap, row
/// flags and the two rejection calls. Eligible rows are counted and selected
/// rows are passed through their row vtable check. The rewrite follows the
/// path where the eligible count stays below the later sort threshold; the
/// sort and per-row action paths are not implemented here.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const MODE: u32 = 0x0117_3604;
const POOL_HANDLE: u32 = 0x012E_22A4;
const COOKIE_SEED: u32 = 0x0105_7FB4;
const MODE_MASK: u32 = 0x3F;
const REQUIRED_MODE: u32 = 0x28;
const MAX_SELECTED: u32 = 100;

fn vcall_thiscall0(stub: u32, this: u32) -> u32 {
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(stub as usize) };
    f(this)
}

export!(cdecl, rw_afff630() -> u32 {
    unsafe { body_afff630(false) }
});

// Mutant: invert the first row rejection answer, which changes whether the
// scan proceeds to the second rejection call and the row's vtable check.
export!(cdecl, mut_afff630() -> u32 {
    unsafe { body_afff630(true) }
});

unsafe fn body_afff630(mutant: bool) -> u32 {
    let mode = global::<u32>(MODE).read() & MODE_MASK;
    if mode == REQUIRED_MODE {
        let pool = global::<u32>(POOL_HANDLE).read();
        if pool != 0 {
            let row_base = (pool as *const u32).read_unaligned();
            let used_base = ((pool + 4) as *const u32).read_unaligned();
            let mut remaining = ((pool + 8) as *const u32).read_unaligned();
            let stride = ((pool + 12) as *const u32).read_unaligned();
            let mut eligible = 0u32;
            let mut selected = 0u32;

            while remaining != 0 {
                remaining = remaining.wrapping_sub(1);
                let used = (used_base.wrapping_add(remaining) as *const u8).read();
                if used & 0x80 != 0 {
                    continue;
                }

                let row = row_base.wrapping_add(stride.wrapping_mul(remaining));
                if row == 0 || *((row + 0x10B8) as *const u8) == 2 {
                    continue;
                }

                let row_flags = *((row + 0xF1F) as *const u8);
                let type_bits = *((row + 0x28) as *const u32) & 0x7C00;
                if row_flags & 0x40 == 0 && type_bits != 0xC00 {
                    let secondary_flags = *((row + 0xF17) as *const u8);
                    if secondary_flags & 8 == 0 {
                        continue;
                    }
                }

                let first_reject = callee_thiscall!(2, u32, row);
                let first_rejects = if mutant {
                    (first_reject as u8) == 0
                } else {
                    (first_reject as u8) != 0
                };
                if first_rejects {
                    continue;
                }

                let second_reject = callee_thiscall!(3, u32, row);
                if (second_reject as u8) != 0 {
                    continue;
                }

                let vtable = (row as *const u32).read_unaligned();
                let row_check = *((vtable + 0x34) as *const u32);
                let checked = vcall_thiscall0(row_check, row);
                if (checked as u8) != 0 && selected < MAX_SELECTED {
                    selected = selected.wrapping_add(1);
                }
                eligible = eligible.wrapping_add(1);
            }

            let recent_count = callee_cdecl!(1, u32,);
            let limit = if (recent_count as u8) != 0 { 8 } else { 10 };
            if eligible > limit {
                // This selected proof ends before distance calculation,
                // sorting and per-row actions.
                let _ = selected;
            }
        }
    }

    let cookie = global::<u32>(COOKIE_SEED).read();
    callee_thiscall!(5, u32, cookie)
}
