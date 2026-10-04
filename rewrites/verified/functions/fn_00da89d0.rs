// original: 0x00da89d0 tracker_position_update
/// Refresh the tracked position when it has moved or the stamp changed.
///
/// Copies the candidate position into the tracker when the squared distance
/// from the stored position exceeds 4.0 (a 2-unit move) or the stored time
/// differs from the new one, marks the tracker touched, then sets flag bit 0
/// to the low bit of the mode byte and returns the flipped bit.
export!(thiscall, rw_00da89d0(this: u32, p: u32, argf: f32, argb: u32) -> u32 { unsafe {
    /// Stored position (x, y, z).
    const PX: u32 = 0x30;
    const PY: u32 = 0x34;
    const PZ: u32 = 0x38;
    /// Stored stamp, copied from the candidate.
    const STAMP: u32 = 0x3c;
    /// Stored time, compared against the new one.
    const TIME: u32 = 0x54;
    /// Tracker flags.
    const FLAGS: u32 = 0x6c;
    /// Squared refresh distance (2 units).
    const MOVE2: f32 = 4.0;
    /// Flag bit set once the tracker has been refreshed.
    const TOUCHED: u32 = 2;
    let dx = f32::from_bits(((this.wrapping_add(PX)) as *const u32).read_unaligned()) - f32::from_bits(((p) as *const u32).read_unaligned());
    let dy = f32::from_bits(((this.wrapping_add(PY)) as *const u32).read_unaligned()) - f32::from_bits(((p.wrapping_add(4)) as *const u32).read_unaligned());
    let dz = f32::from_bits(((this.wrapping_add(PZ)) as *const u32).read_unaligned()) - f32::from_bits(((p.wrapping_add(8)) as *const u32).read_unaligned());
    // Ordered accumulation matching the original's scalar adds; a plain chain
    // would be vectorised or commuted and change NaN payloads.
    let d2 = (core::hint::black_box((core::hint::black_box(dx * dx) + core::hint::black_box(dy * dy))) + core::hint::black_box(dz * dz));
    // ucomiss reports unordered as not-equal, exactly like `!=` on f32.
    if d2 > MOVE2 || f32::from_bits(((this.wrapping_add(TIME)) as *const u32).read_unaligned()) != argf {
        ((this.wrapping_add(PX)) as *mut u32).write_unaligned((f32::from_bits(((p) as *const u32).read_unaligned())).to_bits());
        ((this.wrapping_add(PY)) as *mut u32).write_unaligned((f32::from_bits(((p.wrapping_add(4)) as *const u32).read_unaligned())).to_bits());
        ((this.wrapping_add(PZ)) as *mut u32).write_unaligned((f32::from_bits(((p.wrapping_add(8)) as *const u32).read_unaligned())).to_bits());
        ((this.wrapping_add(STAMP)) as *mut u32).write_unaligned(((p.wrapping_add(0x0c)) as *const u32).read_unaligned());
        ((this.wrapping_add(FLAGS)) as *mut u32).write_unaligned(((this.wrapping_add(FLAGS)) as *const u32).read_unaligned() | TOUCHED);
        ((this.wrapping_add(TIME)) as *mut u32).write_unaligned((argf).to_bits());
    }
    let flags = ((this.wrapping_add(FLAGS)) as *const u32).read_unaligned();
    let flipped = ((argb & 0xFF) ^ flags) & 1;
    ((this.wrapping_add(FLAGS)) as *mut u32).write_unaligned(flags ^ flipped);
    flipped
    } });
