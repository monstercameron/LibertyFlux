// original: 0x0089f5e0 audio_slots_notify
/// Notify every live audio slot and fold the per-slot results.
///
/// Walks the `count` slots at `this+0x48`, skipping empty (`0xff`) slots
/// and slots with no record. Each live slot is announced to callee 1 with
/// the caller's argument, resolved through callee 2 (a null answer, or an
/// answer whose own `+0x48` byte is clear, counts as absent), then handed
/// to callee 3 together with the mode bit. Returns 2 immediately if any
/// callee 3 answers 2, else 0 if any answered 0, else 1. An empty owner
/// (count 0) returns 1 without calling out.
use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

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
export!(thiscall, rw_rb39_f5e0(this: u32, arg0: u32) -> u32 {
    unsafe {
        let count = *((this + 0xd0) as *const u32);
        if count == 0 {
            return 1;
        }
        let mut status: u32 = 1;
        let mut i: u32 = 0;
        while i < count {
            let slot = *((this + 0x48 + i) as *const u8);
            if slot != SLOT_EMPTY {
                let rec = record_for(this, slot);
                if rec != 0 {
                    callee_thiscall!(1, u32, rec, arg0, 0);
                    let bit = mode_bit(this);
                    let key = resolver_key(this);
                    let obj: u32 = callee_cdecl!(2, u32, key);
                    let live = obj != 0 && *((obj + 0x48) as *const u8) != 0;
                    let rec2 = record_for(this, slot);
                    let r: u32 = callee_thiscall!(3, u32, rec2, if live { obj } else { 0 }, bit, 0);
                    if r == 2 {
                        return 2;
                    }
                    if r == 0 {
                        status = 0;
                    }
                }
            }
            i += 1;
        }
        status
    }
});
