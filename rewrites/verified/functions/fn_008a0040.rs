// original: 0x008a0040 audio_params_commit
/// Commit this owner's float parameters, then announce it downstream.
///
/// If `+0xdc` points at a float, its truncated value replaces the slot
/// count at `+0xd0`. If `+0xd8` points at a float, that value (scaled by
/// the shared 1000.0 factor, truncated to 64 bits) replaces the low word
/// at `+0xcc`. The owner is then resolved through callee 1 and announced
/// to callee 2 with its record address and mode bit; only when callee 2
/// answers 1 is the caller's argument forwarded to callee 3 and `+0xc8`
/// cleared. Returns callee 3's answer on the taken path, else callee 2's
/// answer.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

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
/// File VA of the seconds-to-milliseconds float scale in read-only data.
const G_SCALE: u32 = 0xfe8c58;

/// Truncating float-to-i32 conversion with x86 `cvttss2si` semantics: NaN,
/// infinities and out-of-range values all yield `i32::MIN` (Rust's `as`
/// would saturate instead, so the edges are checked explicitly).
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Low 32 bits of a truncating float-to-i64 conversion with x87 `fistp`
/// semantics: unrepresentable values (NaN, infinities, beyond i64 range)
/// store `0x8000_0000_0000_0000`, whose low half is 0.
fn fistp_qword_low(x: f32) -> u32 {
    const LIM: f32 = 9223372036854775808.0; // 2^63, exactly representable
    if x.is_nan() || x >= LIM || x < -LIM {
        0
    } else {
        (x as i64) as u32
    }
}
export!(thiscall, rw_rb39_0040(this: u32, arg0: u32) -> u32 {
    unsafe {
        let p_count = *((this + 0xdc) as *const u32);
        if p_count != 0 {
            let v = *(p_count as *const f32);
            *((this + 0xd0) as *mut u32) = cvttss2si(v) as u32;
        }
        let p_time = *((this + 0xd8) as *const u32);
        if p_time != 0 {
            let v = *(p_time as *const f32);
            let scale = *global::<f32>(G_SCALE);
            *((this + 0xcc) as *mut u32) = fistp_qword_low(v * scale);
        }
        let bit = mode_bit(this);
        let key = resolver_key(this);
        let obj: u32 = callee_cdecl!(1, u32, key);
        let slot = *((this + 0x48) as *const u8);
        let rec = if slot == SLOT_EMPTY {
            0
        } else {
            record_for(this, slot)
        };
        let r: u32 = callee_thiscall!(2, u32, rec, obj, bit, 0);
        if r == 1 {
            let slot2 = *((this + 0x48) as *const u8);
            let rec2 = if slot2 == SLOT_EMPTY {
                0
            } else {
                record_for(this, slot2)
            };
            let r3: u32 = callee_thiscall!(3, u32, rec2, arg0);
            *((this + 0xc8) as *mut u32) = 0;
            return r3;
        }
        r
    }
});
