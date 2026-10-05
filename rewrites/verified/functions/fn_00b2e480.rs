// original: 0x00b2e480 wreck_task_push_pool (proposed)

/// Push every eligible pool entry away from this task's anchor point.
///
/// `this` points to the task's anchor record (four floats at `+0x28` ..
/// `+0x34`, used as two sums scaled by one half). `exclude` is a pool entry
/// to skip, compared by address and never read.
///
/// The pool header is read through a global (base at `+0`, flag bytes at
/// `+4`, entry count at `+8`, entry stride at `+0x0c`); entries are scanned
/// from the last to the first, skipping ones whose flag byte has bit 0x80
/// set. An entry is eligible when it is not `exclude`, the select callee
/// accepts it and the confirm callee (called with the entry and a zero)
/// answers true.
///
/// For an eligible entry with position (x, y) (two floats at `+0x30` past
/// its `+0x20` block), the offset from the scaled anchor is normalised:
/// the inverse length is `1 / sqrt(dy*dy + dx*dx)` when that sum of squares
/// is positive or NaN, and zero otherwise (the original decides this with
/// `ucomiss`/`lahf`/`test`/`jp`, i.e. the parity of the masked flag byte;
/// the rewrite computes the same predicate from the value). Each offset
/// component is then scaled stepwise by that inverse, by the 0.02 and 50.0
/// constants and by the frame-time global; the z lane starts from zero, so
/// it stays a signed zero or NaN through the same steps. The float
/// operation order is the original's.
///
/// The entry's virtual slot at `+0xec` is then called with a scratch block
/// and returns a pointer to three floats; the scaled offsets are added to
/// those (x, y, z in order) and the result is passed to the velocity callee
/// with the entry.
///
/// The return value is whatever the last callee call (or, when no entry
/// called anything, whatever was in eax on entry) left behind; the known
/// callers ignore it, so it is not part of this behaviour. The rewrite
/// returns the last callee answer, or zero when no call fired.
///
/// Original: 0x00b2e480 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b2e480(this: u32, exclude: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x12E22A4;
        const HALF_GLOBAL: u32 = 0xFE8830;
        const ONE_GLOBAL: u32 = 0xFE88E8;
        const SMALL_GLOBAL: u32 = 0xFE8734;
        const BIG_GLOBAL: u32 = 0xFE8B68;
        const FRAME_TIME_GLOBAL: u32 = 0x11735BC;
        const POOL_BASE: u32 = 0x00;
        const POOL_FLAGS: u32 = 0x04;
        const POOL_COUNT: u32 = 0x08;
        const POOL_STRIDE: u32 = 0x0c;
        const ENTRY_POS_BLOCK: u32 = 0x20;
        const ANCHOR_X0: u32 = 0x28;
        const ANCHOR_X1: u32 = 0x2c;
        const ANCHOR_Y0: u32 = 0x30;
        const ANCHOR_Y1: u32 = 0x34;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const VTABLE_SLOT_IMPULSE: u32 = 0xec;
        const FLAG_HIDDEN: u8 = 0x80;
        const C_SELECT: u32 = 1;
        const C_CONFIRM: u32 = 2;
        // contract id 3 (impulse) is reached through the entry vtable.
        const C_VELOCITY: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let half = rdf(lf_checker_rt::relocated(HALF_GLOBAL));
        let one = rdf(lf_checker_rt::relocated(ONE_GLOBAL));
        let small = rdf(lf_checker_rt::relocated(SMALL_GLOBAL));
        let big = rdf(lf_checker_rt::relocated(BIG_GLOBAL));
        let frame_time = rdf(lf_checker_rt::relocated(FRAME_TIME_GLOBAL));

        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let flags = rd32(pool.wrapping_add(POOL_FLAGS));
        let base = rd32(pool.wrapping_add(POOL_BASE));
        let stride = rd32(pool.wrapping_add(POOL_STRIDE));
        let mut tail = 0u32;
        let mut i = rd32(pool.wrapping_add(POOL_COUNT));
        if i != 0 {
            loop {
                i = i.wrapping_sub(1);
                if rd8(flags.wrapping_add(i)) & FLAG_HIDDEN == 0 {
                    let entry = base.wrapping_add(stride.wrapping_mul(i));
                    if entry != 0 && entry != exclude {
                        let sel: u32 =
                            lf_checker_rt::callee_thiscall!(C_SELECT, u32, this, entry);
                        tail = sel;
                        if sel as u8 != 0 {
                            let ok: u32 = lf_checker_rt::callee_thiscall!(
                                C_CONFIRM, u32, this, entry, 0u32
                            );
                            tail = ok;
                            if ok as u8 != 0 {
                                let block = rd32(entry.wrapping_add(ENTRY_POS_BLOCK));
                                let ax = mul(
                                    add(
                                        rdf(this.wrapping_add(ANCHOR_X1)),
                                        rdf(this.wrapping_add(ANCHOR_X0)),
                                    ),
                                    half,
                                );
                                let dx = sub(rdf(block.wrapping_add(POS_X)), ax);
                                let ay = mul(
                                    add(
                                        rdf(this.wrapping_add(ANCHOR_Y1)),
                                        rdf(this.wrapping_add(ANCHOR_Y0)),
                                    ),
                                    half,
                                );
                                let dy = sub(rdf(block.wrapping_add(POS_Y)), ay);
                                // `ucomiss n2,0; lahf; (an instruction of the original); jp` takes
                                // the square-root path exactly when the masked
                                // flag byte has even parity: n2 > 0 or NaN
                                // (and the unreachable n2 < 0, identically).
                                let n2 = add(mul(dy, dy), mul(dx, dx));
                                let (zf, pf) = if n2.is_nan() {
                                    (true, true)
                                } else if n2 == 0.0 {
                                    (true, false)
                                } else {
                                    (false, false)
                                };
                                let masked =
                                    ((zf as u8) << 6) | ((pf as u8) << 2);
                                let inv = if masked.count_ones() % 2 == 0 {
                                    div(one, core::hint::black_box(n2).sqrt())
                                } else {
                                    0.0
                                };
                                let mut vx = mul(inv, dx);
                                let mut vy = mul(inv, dy);
                                let mut vz = mul(inv, 0.0);
                                vx = mul(vx, small);
                                vy = mul(vy, small);
                                vz = mul(vz, small);
                                vx = mul(vx, big);
                                vy = mul(vy, big);
                                vz = mul(vz, big);
                                vx = mul(vx, frame_time);
                                vy = mul(vy, frame_time);
                                vz = mul(vz, frame_time);
                                let mut scratch = [0u32; 3];
                                let impulse: extern "thiscall" fn(u32, u32) -> u32 =
                                    core::mem::transmute(rd32(
                                        rd32(entry).wrapping_add(VTABLE_SLOT_IMPULSE),
                                    )
                                        as usize);
                                let got = impulse(entry, scratch.as_mut_ptr() as u32);
                                tail = got;
                                let out_x = add(rdf(got), vx);
                                let out_y = add(rdf(got.wrapping_add(4)), vy);
                                let out_z = add(rdf(got.wrapping_add(8)), vz);
                                let vel = [out_x.to_bits(), out_y.to_bits(), out_z.to_bits()];
                                tail = lf_checker_rt::callee_thiscall!(
                                    C_VELOCITY,
                                    u32,
                                    entry,
                                    vel.as_ptr() as u32
                                );
                            }
                        }
                    }
                }
                if i == 0 {
                    break;
                }
            }
        }
        tail
    }
});
