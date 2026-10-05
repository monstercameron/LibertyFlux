// original: 0x00d1a340 ped_taskset_update (proposed)

/// Refresh one ped task set from a source matrix, then fan out to four
/// member-update loops.
///
/// `this` points to the task set (about 5 KB: member arrays low, counts and
/// flag bytes near `+0x13a0`, a four-float vector at `+0x13e0`). `src` points
/// to a small matrix the function only reads (offsets `0x00`..`0x38`).
///
/// First the vector is recomputed: for lanes 0, 1 and 3,
/// `out[i] = M[4i]*v0 + M[0x10+4i]*v1 + M[0x20+4i]*v2 + M[0x30+4i]` in that
/// operation order, where `v` is the old vector. Lane 2 is filled from a word
/// the original reads below its own frame (uninitialized stack); the contract
/// defines that fill as zero, so the rewrite stores `0.0`. The float operation
/// order is the original's, pinned through `black_box` helpers.
///
/// Then an intercepted parameterless callee runs on the set, and four loops
/// walk member arrays guarded by per-index flag bytes: counts at
/// `+0x13a0/+0x13a4/+0x13a8/+0x13ac` (signed, re-read every iteration; a
/// non-positive count skips the loop), flags at `+0x13b0`, `+0x13bb`,
/// `+0x13bc`, `+0x13c7` plus the index. A set flag calls that loop's
/// intercepted member callee with the member address
/// (`this+0x90`/`+0x820`/`+0x8e0`/`+0x1280` plus index times
/// `0xb0`/`0xc0`/`0xe0`/`0x120`) in ECX and `src` on the stack. Callee return
/// values are ignored; the index and cursor are reloaded from spill slots
/// after every call.
///
/// The integer return value is whatever the last loop left in EAX: the final
/// cursor of the last loop that ran (loop 1 leaves its count instead), or 0
/// when no loop ran. It is deterministic and compared.
///
/// Original: 0x00d1a340 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d1a340(this: u32, src: u32) -> u32 {
    unsafe {
        const VEC: u32 = 0x13e0;
        const COUNT1: u32 = 0x13a0;
        const COUNT2: u32 = 0x13a4;
        const COUNT3: u32 = 0x13a8;
        const COUNT4: u32 = 0x13ac;
        const FLAG1: u32 = 0x13b0;
        const FLAG2: u32 = 0x13bb;
        const FLAG3: u32 = 0x13bc;
        const FLAG4: u32 = 0x13c7;
        const BASE1: u32 = 0x90;
        const BASE2: u32 = 0x820;
        const BASE3: u32 = 0x8e0;
        const BASE4: u32 = 0x1280;
        const STRIDE1: u32 = 0xb0;
        const STRIDE2: u32 = 0xc0;
        const STRIDE3: u32 = 0xe0;
        const STRIDE4: u32 = 0x120;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let v0 = rdf(this.wrapping_add(VEC));
        let v1 = rdf(this.wrapping_add(VEC + 4));
        let v2 = rdf(this.wrapping_add(VEC + 8));

        // Lane 0: M[0x10]*v1 + M[0]*v0 + M[0x20]*v2 + M[0x30].
        let t0 = mul(rdf(src), v0);
        let mut lane0 = mul(rdf(src.wrapping_add(0x10)), v1);
        lane0 = add(lane0, t0);
        lane0 = add(lane0, mul(rdf(src.wrapping_add(0x20)), v2));
        lane0 = add(lane0, rdf(src.wrapping_add(0x30)));
        // Lane 1: M[0x14]*v1 + M[4]*v0 + M[0x24]*v2 + M[0x34].
        let mut lane1 = mul(rdf(src.wrapping_add(0x14)), v1);
        lane1 = add(lane1, mul(rdf(src.wrapping_add(4)), v0));
        lane1 = add(lane1, mul(rdf(src.wrapping_add(0x24)), v2));
        lane1 = add(lane1, rdf(src.wrapping_add(0x34)));
        // Lane 3: M[0x18]*v1 + M[8]*v0 + M[0x28]*v2 + M[0x38].
        let mut lane3 = mul(rdf(src.wrapping_add(0x18)), v1);
        lane3 = add(lane3, mul(rdf(src.wrapping_add(8)), v0));
        lane3 = add(lane3, mul(rdf(src.wrapping_add(0x28)), v2));
        lane3 = add(lane3, rdf(src.wrapping_add(0x38)));

        wrf(this.wrapping_add(VEC), lane0);
        wrf(this.wrapping_add(VEC + 4), lane1);
        // Lane 2: the original copies a word from below its frame
        // (uninitialized stack); the contract fills that word with zero.
        wrf(this.wrapping_add(VEC + 12), 0.0);
        wrf(this.wrapping_add(VEC + 8), lane3);

        lf_checker_rt::callee_thiscall!(1, u32, this);

        let mut ret: u32 = 0;
        // Loop 1: index in EAX, cursor in ECX, leaves its count in EAX.
        if (rd32(this.wrapping_add(COUNT1)) as i32) > 0 {
            let mut cursor = this.wrapping_add(BASE1);
            let mut i: i32 = 0;
            loop {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG1)) != 0 {
                    lf_checker_rt::callee_thiscall!(2, u32, cursor, src);
                }
                i = i.wrapping_add(1);
                cursor = cursor.wrapping_add(STRIDE1);
                if !(i < rd32(this.wrapping_add(COUNT1)) as i32) {
                    ret = i as u32;
                    break;
                }
            }
        }
        // Loops 2-4: index in EDI, cursor in EAX, leave the cursor in EAX.
        if (rd32(this.wrapping_add(COUNT2)) as i32) > 0 {
            let mut cursor = this.wrapping_add(BASE2);
            let mut i: i32 = 0;
            loop {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG2)) != 0 {
                    lf_checker_rt::callee_thiscall!(3, u32, cursor, src);
                }
                i = i.wrapping_add(1);
                cursor = cursor.wrapping_add(STRIDE2);
                if !(i < rd32(this.wrapping_add(COUNT2)) as i32) {
                    ret = cursor;
                    break;
                }
            }
        }
        if (rd32(this.wrapping_add(COUNT3)) as i32) > 0 {
            let mut cursor = this.wrapping_add(BASE3);
            let mut i: i32 = 0;
            loop {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG3)) != 0 {
                    lf_checker_rt::callee_thiscall!(4, u32, cursor, src);
                }
                i = i.wrapping_add(1);
                cursor = cursor.wrapping_add(STRIDE3);
                if !(i < rd32(this.wrapping_add(COUNT3)) as i32) {
                    ret = cursor;
                    break;
                }
            }
        }
        if (rd32(this.wrapping_add(COUNT4)) as i32) > 0 {
            let mut cursor = this.wrapping_add(BASE4);
            let mut i: i32 = 0;
            loop {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG4)) != 0 {
                    lf_checker_rt::callee_thiscall!(5, u32, cursor, src);
                }
                i = i.wrapping_add(1);
                cursor = cursor.wrapping_add(STRIDE4);
                if !(i < rd32(this.wrapping_add(COUNT4)) as i32) {
                    ret = cursor;
                    break;
                }
            }
        }
        ret
    }
});
