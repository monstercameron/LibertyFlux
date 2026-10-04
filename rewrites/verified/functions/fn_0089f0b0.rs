// original: 0x0089f0b0 audio_best_under
/// Find the smallest answer beneath this owner's slots.
///
/// For each live slot (selector not `0xff`, record address not null),
/// callee 1 fills a 16-entry table through a frame buffer. Every entry
/// that is non-null, tagged 2 at `+6`, has a live sub-selector and a
/// non-null recomputed record is asked through callee 2 with the caller's
/// argument, and the signed minimum of all answers is kept. Returns that
/// minimum, starting from the `+0xb8` word: with no answers at all (or
/// no live slot, or an empty owner) it returns `+0xb8` unchanged.
use lf_k2_rt::{callee_thiscall, export, global};

/// File VA of the global record-table stride (a plain value).
const G_STRIDE: u32 = 0x115d964;
/// File VA of the global record-table base pointer.
const G_TABLE: u32 = 0x115d988;
/// Byte distance between consecutive table rows.
const ROW_STRIDE: u32 = 0x6f40;
/// Offset of the first row cell from the table base.
const ROW_BASE: u32 = 0x6f10;
/// Slot selector meaning "slot empty".
const SLOT_EMPTY: u8 = 0xff;
/// Tag word marking a live sub-entry.
const F2_TAG: u16 = 2;

unsafe fn stride() -> u32 {
    *global::<u32>(G_STRIDE)
}

unsafe fn table_base() -> u32 {
    *global::<u32>(G_TABLE)
}

unsafe fn row_cell(row: u8) -> u32 {
    let addr = table_base()
        .wrapping_add(ROW_BASE)
        .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE));
    *(addr as *const u32)
}

/// Record address for one slot: `row_cell + stride * slot`.
unsafe fn record_for(this: u32, slot: u8) -> u32 {
    let row = *((this + 0x40) as *const u8);
    row_cell(row).wrapping_add(stride().wrapping_mul(slot as u32))
}

/// Mode bit carried in bit 5 of the flag byte at `+0x39`.
unsafe fn mode_bit(this: u32) -> u32 {
    (((*((this + 0x39) as *const u8)) >> 5) & 1) as u32
}

/// Sign-extended resolver key from `+0x3c`.
unsafe fn resolver_key(this: u32) -> u32 {
    *((this + 0x3c) as *const i16) as i32 as u32
}
export!(thiscall, rw_rb39_f0b0(this: u32, arg0: u32) -> u32 {
    unsafe {
        let count = *((this + 0xd0) as *const u32);
        let entry_b8 = *((this + 0xb8) as *const u32);
        if count == 0 {
            return entry_b8;
        }
        let mut best: i32 = 0x7fffffff;
        let mut m: u32 = entry_b8;
        let mut i: u32 = 0;
        while i < count {
            let s = *((this + 0x48 + i) as *const u8);
            if s != SLOT_EMPTY {
                let rec = record_for(this, s);
                if rec != 0 {
                    let mut out = [0u32; 16];
                    callee_thiscall!(1, u32, rec, 0xc, out.as_mut_ptr() as u32, 0x10, 1);
                    for j in 0..16 {
                        let e = out[j];
                        if e == 0 {
                            continue;
                        }
                        if *((e + 6) as *const u16) != F2_TAG {
                            continue;
                        }
                        let b = *((e + 0x48) as *const u8);
                        if b == SLOT_EMPTY {
                            continue;
                        }
                        let r = *((e + 0x40) as *const u8);
                        let rec2 =
                            row_cell(r).wrapping_add(stride().wrapping_mul(b as u32));
                        if rec2 == 0 {
                            continue;
                        }
                        let a: u32 = callee_thiscall!(2, u32, rec2, arg0);
                        if (a as i32) < best {
                            best = a as i32;
                            m = a;
                        }
                    }
                }
            }
            i += 1;
        }
        m
    }
});
