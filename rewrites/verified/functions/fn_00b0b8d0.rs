// original: 0x00b0b8d0 net_rows_in_radius (proposed)

/// Run the radius callback over every active table row inside the radius.
///
/// Scans all 1500 rows of the slot table. A row is considered when its
/// active byte (`+0x28`) is non-zero and its skip bit (`+0x29` bit 0) is
/// clear. `mode` then filters by the row's id word (`+0x24`): mode 0 keeps
/// rows whose id equals either of the two wanted ids, mode 1 asks the row's
/// registered checker (reached through the id-indexed table, virtual slot
/// `+0xc`) and keeps the row when it answers 4, mode 2 keeps rows whose id
/// equals the wanted id. Any other mode keeps nothing.
///
/// A kept row whose (x, y) position (`-0x4`, `+0x0`) lies strictly within
/// `radius` of `pos` is offered to the handle checker with its stored handle
/// (`-0xc`); when that answers zero the row's position triple and index go
/// to the emit callee, the row's seen bit (`+0x29` bit 5) is set and the
/// emitted handle is stored back at `-0xc`. The squared radius is kept in
/// the incoming radius argument slot, so the stack comparison is off for
/// this function.
///
/// Original: 0x00b0b8d0 (cdecl, three stack words).
export!(cdecl, rw_00b0b8d0(pos: u32, radius: u32, mode: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x161567c;
        const ROWS: u32 = 1500;
        const ROW_STRIDE: u32 = 80;
        const OFF_HANDLE: i32 = -0x0c;
        const OFF_X: i32 = -4;
        const OFF_Y: i32 = 0;
        const OFF_Z: i32 = 4;
        const OFF_ID: u32 = 0x24;
        const OFF_ACTIVE: u32 = 0x28;
        const OFF_FLAGS: u32 = 0x29;
        const SKIP_BIT: u8 = 0x01;
        const SEEN_BIT: u8 = 0x20;
        const WANT_A: u32 = 0x12f9da4;
        const WANT_B: u32 = 0x12f9db0;
        const WANT_C: u32 = 0x12fa3e0;
        const ID_TABLE: u32 = 0x1295cd8;
        const VT_CHECK: u32 = 0x0c;
        const KEEP_ANSWER: u8 = 4;
        const EMIT_FMT: u32 = 0xeaa8e4;
        const C_CHECK: u32 = 1;
        const C_HANDLE: u32 = 2;
        const C_EMIT: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
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
        unsafe fn row_addr(row: u32, off: i32) -> u32 {
            unsafe { lf_checker_rt::relocated(TABLE)
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(off as u32) }
        }

        let r = f32::from_bits(radius);
        let r2 = mul(r, r);
        let px = rdf(pos);
        let py = rdf(pos + 4);
        let mut idx: u32 = 0;
        while idx < ROWS {
            let active = (row_addr(idx, OFF_ACTIVE as i32) as *const u8).read();
            let flags = (row_addr(idx, OFF_FLAGS as i32) as *const u8).read();
            if active != 0 && flags & SKIP_BIT == 0 {
                let id = ((row_addr(idx, OFF_ID as i32) as *const u16).read_unaligned() as i16) as i32;
                let keep = match mode {
                    0 => {
                        let a = g32(WANT_A) as i32;
                        let b = g32(WANT_B) as i32;
                        id == a || id == b
                    }
                    1 => {
                        let slot = (lf_checker_rt::relocated(ID_TABLE) as i32)
                            .wrapping_add(id.wrapping_mul(4)) as u32;
                        let thisp = rd32(slot);
                        let vt = rd32(thisp);
                        let check: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + VT_CHECK) as usize);
                        (check(thisp) as u8) == KEEP_ANSWER
                    }
                    2 => id == g32(WANT_C) as i32,
                    _ => false,
                };
                if keep {
                    let dx = sub(rdf(row_addr(idx, OFF_X)), px);
                    let dy = sub(rdf(row_addr(idx, OFF_Y)), py);
                    let d2 = add(mul(dy, dy), mul(dx, dx));
                    if d2 < r2 {
                        let h = rd32(row_addr(idx, OFF_HANDLE));
                        let seen: u32 = lf_checker_rt::callee_cdecl!(C_HANDLE, u32, h);
                        if (seen as u8) == 0 {
                            let mut triple = [0u32; 3];
                            triple[0] = rd32(row_addr(idx, OFF_X));
                            triple[1] = rd32(row_addr(idx, OFF_Y));
                            triple[2] = rd32(row_addr(idx, OFF_Z));
                            let out: u32 = lf_checker_rt::callee_cdecl!(
                                C_EMIT, u32, triple.as_mut_ptr() as u32,
                                lf_checker_rt::relocated(EMIT_FMT), idx);
                            let fp = row_addr(idx, OFF_FLAGS as i32) as *mut u8;
                            fp.write(fp.read() | SEEN_BIT);
                            wr32(row_addr(idx, OFF_HANDLE), out);
                        }
                    }
                }
            }
            idx += 1;
        }
        0
    }
});
