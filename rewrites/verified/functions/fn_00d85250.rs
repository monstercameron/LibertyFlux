// original: 0x00D85250 input_ui_track_update
/// Advance one input-tracked slot toward its target entry, then delegate.
///
/// The object byte picks a table slot. A negative pick, an empty slot, or
/// the end of the entry run all zero the four outputs. A flagged slot
/// reports the half-step default instead. Otherwise the entry cursor walks
/// forward in 32-byte steps while the near distance is under ten units or
/// still shrinking, the winning entry's packed direction scales the slot
/// rate, and the stored solver call finishes the update.
///
/// Returns the solver's answer on the main path, the flag pointer from the
/// comparison against it on the early-out paths.
use lf_checker_rt::{callee_cdecl, export, global, relocated};

/// Truncating float-to-int conversion with x86 \cvttss2si\ semantics:
/// round toward zero; NaN or out-of-range yields \i32::MIN\ (it never
/// saturates, unlike a Rust \s\ cast).
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}

export!(cdecl, rw_d85250(obj: u32, p1: u32, p2: u32, p3: u32, pflag: u32, extra: u32) -> u32 {
    /// Per-slot entry base offsets; zero means the slot is empty.
    const G_BASE: u32 = 0x0165_76B0;
    /// Per-slot entry cursor, advanced 32 bytes per step.
    const G_CURSOR: u32 = 0x0165_7710;
    /// Per-slot entry run end.
    const G_END: u32 = 0x0165_7770;
    /// Per-slot rate multiplier.
    const G_RATE: u32 = 0x0165_7830;
    /// Per-slot ready flags; set means report the default.
    const G_READY: u32 = 0x0165_78C0;
    /// Near-distance limit that keeps the cursor walking.
    const NEAR_LIMIT: f32 = 10.0;
    /// Floor for the scaled rate on the short-cursor path.
    const RATE_FLOOR: f32 = 5.0;
    /// Packed-direction unit scale.
    const DIR_SCALE: f32 = 0.0036622213665395975;
    /// Half-step default reported for a flagged slot.
    const HALF_STEP: u32 = 0x3F00_0000;
    /// Cursor stride in bytes.
    const STRIDE: u32 = 0x20;

    #[inline(always)]
    fn zero_out(p1: u32, p2: u32, p3: u32, pflag: u32) {
        unsafe {
            (p3 as *mut u32).write(0);
            (p2 as *mut u32).write(0);
            (p1 as *mut u32).write(0);
            (pflag as *mut u8).write(0);
        }
    }
    #[inline(always)]
    fn read_f32(addr: u32) -> f32 {
        unsafe { (addr as *const f32).read() }
    }

    let idx = unsafe { ((obj + 0xF10) as *const i8).read() } as i32;
    if idx < 0 {
        zero_out(p1, p2, p3, pflag);
        return pflag;
    }
    let slot = idx as u32 as usize;
    let edi = unsafe { global::<u32>(G_BASE).add(slot).read() };
    if edi == 0 {
        zero_out(p1, p2, p3, pflag);
        return pflag;
    }
    if unsafe { ((relocated(G_READY) + slot as u32) as *const u8).read() } != 0 {
        unsafe {
            (p2 as *mut u32).write(0);
            (p1 as *mut u32).write(0);
            (p3 as *mut u32).write(HALF_STEP);
            (pflag as *mut u8).write(0);
        }
        return pflag;
    }
    let anchor = unsafe { ((obj + 0x20) as *const u32).read() };
    let cursor_cell = unsafe { global::<u32>(G_CURSOR).add(slot) };
    let first = unsafe { cursor_cell.read() }.wrapping_add(edi);
    let saved = first;
    let mut cur = first;
    loop {
        // Near distance to the entry head, then to the entry tail.
        let dx = read_f32(anchor + 0x30) - read_f32(cur + 0x14);
        let dy = read_f32(anchor + 0x34) - read_f32(cur + 0x18);
        let dz = read_f32(anchor + 0x38) - read_f32(cur + 0x1C);
        let d1 = ((dy * dy + dx * dx) + dz * dz).sqrt();
        let dx2 = read_f32(anchor + 0x30) - read_f32(cur + 0x34);
        let dy2 = read_f32(anchor + 0x34) - read_f32(cur + 0x38);
        let dz2 = read_f32(anchor + 0x38) - read_f32(cur + 0x3C);
        let d2 = ((dy2 * dy2 + dx2 * dx2) + dz2 * dz2).sqrt();
        // Exit when the head is past the near limit and not nearer than
        // the tail; each comparison is false for NaN, matching the jumps.
        if !(NEAR_LIMIT > d1) && !(d1 > d2) {
            break;
        }
        let advanced = unsafe { cursor_cell.read() }.wrapping_add(STRIDE);
        unsafe { cursor_cell.write(advanced) };
        let bound = unsafe { global::<u32>(G_END).add(slot).read() }.wrapping_sub(STRIDE);
        if (advanced as i32) >= (bound as i32) {
            callee_cdecl!(2, u32, idx as u32);
            unsafe { ((obj + 0xF10) as *mut u8).write(0xFF) };
            zero_out(p1, p2, p3, pflag);
            return pflag;
        }
        cur = advanced.wrapping_add(edi);
    }
    let g3val = unsafe { cursor_cell.read() };
    let short_x = unsafe { ((saved + 4) as *const i16).read() } as f32 * DIR_SCALE;
    let short_y = unsafe { ((saved + 6) as *const i16).read() } as f32 * DIR_SCALE;
    let mut rate = (short_x * short_x + short_y * short_y).sqrt()
        * read_f32(relocated(G_RATE) + slot as u32 * 4);
    // Dead in practice (a live cursor is always past 0x320) but kept exact.
    if !((g3val as i32) > 0x320) && !(rate > RATE_FLOOR) {
        rate = RATE_FLOOR;
    }
    let count = cvtt_ss2si(rate);
    let kept: i32 = if (count as i8) > 1 {
        (count as i8) as i32
    } else {
        1
    };
    unsafe { ((obj + 0xE6F) as *mut u8).write(kept as u8) };
    let fx = unsafe { ((saved + 0x14) as *const u32).read() };
    let fy = unsafe { ((saved + 0x18) as *const u32).read() };
    callee_cdecl!(
        1,
        u32,
        obj,
        0,
        fx,
        fy,
        p1,
        p2,
        p3,
        pflag,
        extra,
        f32::to_bits((kept as u8) as f32)
    )
});

