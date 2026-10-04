// original: 0x00a654f0 resolve_handle_pair
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Resolve one handle pair into its output vector through the type table.
///
/// Gates on two selector bytes, then on the pair's kind and state words,
/// falling back to a compatibility probe when the fast checks disagree. A
/// five-word query resolves the pair to an index; the index walks a
/// two-stage lookup (a per-object step, then the shared step) whose result
/// is published to the integer slot. The shared step's answer indexes the
/// global type table, and the row found there is transformed twice through
/// frame scratch: the second transform fills four words that are copied to
/// the output vector. Returns 1 with the outputs stored, or 0 leaving them
/// alone.
export!(cdecl, rw_00a654f0(pair_a: u32, pair_b: u32, vec_out: u32, int_out: u32) -> u32 {
    unsafe {
        let edi = pair_a;
        let esi = pair_b;
        let mut hot = false;
        if ((edi + 0x218) as *const u8).read() == 0
            && ((edi + 0x219) as *const u8).read() != 0
        {
            if ((esi + 0x10B8) as *const u32).read() == 2 {
                hot = true;
            } else {
                let compat = callee_cdecl!(1, u32, edi, esi);
                if (compat as u8) != 0 {
                    hot = true;
                } else if ((esi + 0x1300) as *const u32).read() == 2 {
                    hot = true;
                }
            }
        } else if ((esi + 0x1300) as *const u32).read() == 2 {
            hot = true;
        }
        // The original pushes the hot byte slot as a full word; the upper
        // bytes are pristine stack fill (zero) on both sides.
        let hot_word: u32 = if hot { 1 } else { 0 };
        let class: u32 = if ((esi + 0x1304) as *const u32).read() == 3 {
            0xFFFF_FFFA
        } else {
            0xFFFF_FFFB
        };
        let index = callee_cdecl!(2, u32, edi, esi, class, hot_word, 0);
        if index == 0xFFFF_FFFF {
            return 0;
        }
        let step = callee_thiscall!(3, u32, esi, index);
        let shared = callee_thiscall!(4, u32, step, esi, 0);
        (int_out as *mut u32).write(shared);
        let kind = ((esi + 0x2E) as *const i16).read() as i32;
        let table = global::<u32>(0x01295CD8);
        let row0 = table.offset(kind as isize).read() as u32;
        let rows = ((row0 + 0xCC) as *const u32).read();
        let row = ((rows + shared.wrapping_mul(4)) as *const u32).read();
        if row == 0xFFFF_FFFF {
            return 0;
        }
        let shaped = callee_thiscall!(5, u32, esi, row);
        let mut first = [0u32; 24];
        callee_thiscall!(6, u32, first.as_mut_ptr() as u32, shaped);
        let mut second = [0u32; 24];
        let tag = ((esi + 0x20) as *const u32).read();
        // The stub fills words 12..16 of the second buffer (the original's
        // frame words the transform writes); the first buffer's address is
        // passed through exactly like the original and skipped.
        callee_thiscall!(
            7,
            u32,
            second.as_mut_ptr() as u32,
            first.as_mut_ptr() as u32,
            tag
        );
        (vec_out as *mut u32).write(second[12]);
        ((vec_out + 4) as *mut u32).write(second[13]);
        ((vec_out + 8) as *mut u32).write(second[14]);
        ((vec_out + 12) as *mut u32).write(second[15]);
        1
    }
});
