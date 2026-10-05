// original: 0x00a9ee60 stream_slot_in_radius (proposed)

/// Report whether any active slot with a matching id lies within a radius.
///
/// `this` points to an object holding 256 slots of 0x30 bytes at `+0x40d8`.
/// Slot `i` is a candidate when its flag byte (slot `-0x18`) is non-zero
/// and its id word (slot `-0x14`) equals `id`. For a candidate the squared
/// distance from the point `pos` (three floats) to the slot centre (floats
/// at slot `-8`, `-4`, `+0`, i.e. x, y, z) is formed exactly as the
/// original's scalar SSE chain does: `dy*dy + dx*dx + dz*dz` with the two
/// additions in that order, and compared against `radius * radius`. The
/// first candidate inside (comparison ordered, `r2 >= d2`) ends the scan.
/// The result is 1 when a slot matched and 0x100 when none did.
///
/// Float operation order is pinned through `black_box` helpers.
///
/// Original: 0x00a9ee60 (thiscall, three stack words: id, pos pointer,
/// radius bits).
lf_checker_rt::export!(thiscall, rw_00a9ee60(this: u32, id: u32, pos: u32, radius: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0x40d8;
        const SLOT_STRIDE: u32 = 0x30;
        const SLOT_COUNT: u32 = 0x100;
        const FLAG_DELTA: i32 = -0x18;
        const ID_DELTA: i32 = -0x14;
        const CX_DELTA: i32 = -8;
        const CY_DELTA: i32 = -4;
        const CZ_DELTA: i32 = 0;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }

        let r = f32::from_bits(radius);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let s = this
                .wrapping_add(SLOTS_OFF)
                .wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            let live = (s.wrapping_add_signed(FLAG_DELTA) as *const u8).read() != 0;
            let same = (s.wrapping_add_signed(ID_DELTA) as *const u32).read_unaligned() == id;
            if live && same {
                // Loaded per candidate like the original: hoisting these
                // above the loop would fault on trials the original survives.
                let px = rdf(pos);
                let py = rdf(pos + 4);
                let pz = rdf(pos + 8);
                let dx = sub(px, rdf(s.wrapping_add_signed(CX_DELTA)));
                let dy = sub(py, rdf(s.wrapping_add_signed(CY_DELTA)));
                let dz = sub(pz, rdf(s.wrapping_add_signed(CZ_DELTA)));
                let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if mul(r, r) >= d2 {
                    return 1;
                }
            }
            i += 1;
        }
        0x100
    }
});
