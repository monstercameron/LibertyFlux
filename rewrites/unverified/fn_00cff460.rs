// original: 0x00cff460 task_flag_by_forward_dot (proposed)

/// Sets bit 0x20 of the flag byte at `this + 0x35` when the target point
/// lies behind the task's facing plane; always clears the bit first.
///
/// `this` (ECX) holds a point in three floats at `+0x20`, `+0x24`, `+0x28`.
/// `obj` points to a record whose dword at `+0xd68` leads to a mode word and
/// whose pointer at `+0x20` leads to a plane (normal at `+0x10`, `+0x14`,
/// `+0x18`, origin at `+0x30`, `+0x34`, `+0x38`). Bit 0x20 of the flag byte
/// is cleared on entry. When mode bits 3..4 (word `>> 3 & 3`) are 3 or more
/// the function stops there. Otherwise it forms the dot product of the plane
/// normal with (point - origin) in single precision, in the original's
/// operation order, and sets bit 0x20 only when 0.0 is strictly greater than
/// the dot product (a NaN dot product leaves the bit clear, matching the
/// original's unordered-compare skip). No return value.
///
/// Original: 0x00cff460 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cff460(this: u32, obj: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x35;
        const DOT_BIT: u8 = 0x20;
        const PX: u32 = 0x20;
        const PY: u32 = 0x24;
        const PZ: u32 = 0x28;
        const MODE_SLOT: u32 = 0xD68;
        const PLANE_SLOT: u32 = 0x20;
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
        let fb = this.wrapping_add(FLAGS) as *mut u8;
        let mut flags = fb.read();
        flags &= !DOT_BIT;
        fb.write(flags);
        let mode_ptr = (obj.wrapping_add(MODE_SLOT) as *const u32).read_unaligned();
        let mode = ((mode_ptr as *const u32).read_unaligned() >> 3) & 3;
        if mode >= 3 {
            return 0;
        }
        let plane = (obj.wrapping_add(PLANE_SLOT) as *const u32).read_unaligned();
        let dx = sub(rdf(this.wrapping_add(PX)), rdf(plane.wrapping_add(0x30)));
        let dy = sub(rdf(this.wrapping_add(PY)), rdf(plane.wrapping_add(0x34)));
        let dz = sub(rdf(this.wrapping_add(PZ)), rdf(plane.wrapping_add(0x38)));
        let t1 = mul(rdf(plane.wrapping_add(0x14)), dy);
        let t0 = mul(rdf(plane.wrapping_add(0x10)), dx);
        let mut dot = add(t1, t0);
        dot = add(dot, mul(rdf(plane.wrapping_add(0x18)), dz));
        if 0.0f32 > dot {
            fb.write(flags | DOT_BIT);
        }
        0
    }
});
