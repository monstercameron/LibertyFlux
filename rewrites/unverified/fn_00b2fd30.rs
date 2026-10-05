// original: 0x00b2fd30 ped_task_collect_typed_records (proposed)

/// Scan a 32-entry global source table and collect nearby typed records.
///
/// The table at `TABLE_BASE` holds 32 records of `RECORD_STRIDE` bytes; each
/// record's first dword is a type tag. Records tagged `TAG_A`/`TAG_B`/`TAG_C`
/// are each handed (record address as `this`, one of three scratch buffers)
/// to the fetch callee, which answers a pointer to four floats. A fetched
/// point is kept when its squared distance to the anchor point (three floats
/// at `anchor + 0x30`, where `anchor` is the dword at `obj + 0x20`) is
/// strictly below the squared `radius` (strict `>` on the squares, so NaN on
/// either side rejects). Kept points are appended to `out` as `OUT_STRIDE`
/// byte entries: the four fetched floats, then the constants `K0..K3`.
/// Stops after `max` entries (signed comparison, checked after each append)
/// or at the table end. Returns the entry count.
///
/// Original: cdecl, four stack words (object, radius bits, out buffer,
/// signed limit), record addresses passed to the callee unmodified.
lf_checker_rt::export!(cdecl, rw_00b2fd30(obj: u32, radius_bits: u32, out: u32, max: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x0166_1A60;
        const TABLE_END: u32 = 0x0166_2460;
        const RECORD_STRIDE: u32 = 0x50;
        const TAG_A: u32 = 0x11;
        const TAG_B: u32 = 0x12;
        const TAG_C: u32 = 0x1B;
        const OUT_STRIDE: u32 = 0x20;
        const ANCHOR_SLOT: u32 = 0x20;
        const ANCHOR_X: u32 = 0x30;
        const ANCHOR_Y: u32 = 0x34;
        const ANCHOR_Z: u32 = 0x38;
        const K0: u32 = 0x4080_0000; // 4.0
        const K1: u32 = 0x4180_0000; // 16.0
        const K2: u32 = 0x43FA_0000; // 500.0
        const K3: u32 = 0x447A_0000; // 1000.0
        const CALLEE_FETCH: u32 = 1;

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

        let radius = f32::from_bits(radius_bits);
        let mut count: u32 = 0;
        let mut dest = out.wrapping_add(8);
        let mut buf_a = [0u32; 4];
        let mut buf_b = [0u32; 4];
        let mut buf_c = [0u32; 4];
        let mut rec = TABLE_BASE;
        loop {
            let tag = (lf_checker_rt::global::<u32>(rec) as *const u32).read_unaligned();
            let buf = match tag {
                TAG_A => buf_a.as_mut_ptr(),
                TAG_B => buf_b.as_mut_ptr(),
                TAG_C => buf_c.as_mut_ptr(),
                _ => core::ptr::null_mut(),
            };
            if !buf.is_null() {
                let fetched: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_FETCH,
                    u32,
                    lf_checker_rt::relocated(rec),
                    buf as u32
                );
                let v0 = (fetched as *const f32).read_unaligned();
                let v1 = (fetched.wrapping_add(4) as *const f32).read_unaligned();
                let v2 = (fetched.wrapping_add(8) as *const f32).read_unaligned();
                let v3 = (fetched.wrapping_add(12) as *const f32).read_unaligned();
                let anchor =
                    (obj.wrapping_add(ANCHOR_SLOT) as *const u32).read_unaligned();
                let dx = sub(
                    v0,
                    (anchor.wrapping_add(ANCHOR_X) as *const f32).read_unaligned(),
                );
                let dy = sub(
                    v1,
                    (anchor.wrapping_add(ANCHOR_Y) as *const f32).read_unaligned(),
                );
                let dz = sub(
                    v2,
                    (anchor.wrapping_add(ANCHOR_Z) as *const f32).read_unaligned(),
                );
                let mut dist = add(mul(dy, dy), mul(dx, dx));
                dist = add(dist, mul(dz, dz));
                let limit = mul(radius, radius);
                // Original is comiss+jbe: keep only on strict greater-than,
                // which also rejects NaN on either side.
                if limit > dist {
                    (dest.wrapping_sub(8) as *mut f32).write_unaligned(v0);
                    (dest.wrapping_sub(4) as *mut f32).write_unaligned(v1);
                    (dest as *mut f32).write_unaligned(v2);
                    (dest.wrapping_add(4) as *mut f32).write_unaligned(v3);
                    count = count.wrapping_add(1);
                    (dest.wrapping_add(8) as *mut u32).write_unaligned(K0);
                    (dest.wrapping_add(12) as *mut u32).write_unaligned(K1);
                    (dest.wrapping_add(16) as *mut u32).write_unaligned(K2);
                    (dest.wrapping_add(20) as *mut u32).write_unaligned(K3);
                    dest = dest.wrapping_add(OUT_STRIDE);
                    if (count as i32) >= (max as i32) {
                        break;
                    }
                }
            }
            rec = rec.wrapping_add(RECORD_STRIDE);
            if (rec as i32) >= (TABLE_END as i32) {
                break;
            }
        }
        count
    }
});
