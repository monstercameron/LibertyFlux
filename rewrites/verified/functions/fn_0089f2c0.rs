// original: 0x0089f2c0 audio_slots_rebuild
/// Rebuild every slot selector from the shared resolver.
///
/// Copies the caller's 24-byte buffer aside (the resolver receives it and
/// may clobber it), then resolves each of the `count` slots through callee
/// 1: a null answer means slot empty (`0xff`), otherwise the slot byte is
/// the low byte of `(answer - row_cell) / stride`. Slots whose recomputed
/// record address is null also fail. Each success restores the saved
/// buffer and, for the first two slots, records one word from the argument
/// array into `+0x9c`. Returns 1 when every slot resolved, 0 if any failed;
/// a count above 8 is rejected with 0 and an empty owner succeeds with 1.
/// Only the low byte of the return is meaningful (the contract compares
/// `al`: the upper bytes are instruction leftovers on some paths).
use lf_k2_rt::{callee_thiscall, export, global, relocated};

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
/// File VA of the fixed owner object the rebuild callee operates on.
const FIXED_OWNER: u32 = 0x115dc18;

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
export!(thiscall, rw_rb39_f2c0(this: u32, buf: u32, arr: u32) -> u32 {
    unsafe {
        let saved = [
            *(buf as *const u32),
            *((buf + 4) as *const u32),
            *((buf + 8) as *const u32),
            *((buf + 12) as *const u32),
            *((buf + 16) as *const u32),
            *((buf + 20) as *const u32),
        ];
        let count = *((this + 0xd0) as *const u32);
        if count > 8 {
            return 0;
        }
        if count == 0 {
            return 1;
        }
        let owner = relocated(FIXED_OWNER);
        let mut ok: u8 = 1;
        let mut i: u32 = 0;
        while i < count {
            let w = *((arr + 5 + i * 8) as *const u32);
            let ans: u32 = callee_thiscall!(1, u32, owner, w, this, this + 0x3c, buf);
            let row = *((this + 0x40) as *const u8);
            let slot: u8 = if ans == 0 {
                SLOT_EMPTY
            } else {
                (ans.wrapping_sub(row_cell(row)) / stride()) as u8
            };
            *((this + 0x48 + i) as *mut u8) = slot;
            let mut good = slot != SLOT_EMPTY;
            if good {
                let rec = row_cell(row).wrapping_add(stride().wrapping_mul(slot as u32));
                if rec == 0 {
                    good = false;
                } else {
                    if i < 2 {
                        *((this + 0x9c + i * 4) as *mut u32) =
                            *((arr + 9 + i * 8) as *const u32);
                    }
                    *(buf as *mut u32) = saved[0];
                    *((buf + 4) as *mut u32) = saved[1];
                    *((buf + 8) as *mut u32) = saved[2];
                    *((buf + 12) as *mut u32) = saved[3];
                    *((buf + 16) as *mut u32) = saved[4];
                    *((buf + 20) as *mut u32) = saved[5];
                }
            }
            if !good {
                ok = 0;
            }
            i += 1;
        }
        ok as u32
    }
});
