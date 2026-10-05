// original: 0x00d916c0 audio_navmesh_load
/// Load a NAVM audio-geometry section from a file into a new object.
///
/// `path` is a NUL-terminated file name; a null or empty path returns 0.
/// The file is opened through callee 1 and must start with the four magic
/// bytes `NAVM`, followed by a version dword. A null handle or a magic
/// mismatch closes the file (when opened) and returns 0.
///
/// Callee 7 builds the section object. The loader then reads the flag
/// word (`+0x50`), a spare dword (`+0x80`), resolves the coefficient block
/// through callee 10 (copied to the `+0x90` block when non-null), derives
/// the float extents (`+0x40..0x48`) from it, and reads the entry count
/// (`+0x78`) and the cell count (`+0x7c`). A nonzero global byte forces the
/// flag's low bit off, selecting the dword entry path over the packed
/// triplet path.
///
/// Entries are allocated through callee 13 (count*3 u16 triplets, read two
/// bytes at a time) or callee 17 (count dwords of 16 bytes, read straight
/// into the buffer). A further count feeds the thread-local allocator
/// (callee 22, reached through the TLS slot named by a global), then two
/// more buffers are allocated (callees 23 and 24: per-cell corner data and
/// the 40-byte cell records).
///
/// Each cell record is decoded from two bytes (low 21 bits and bits 21-24
/// of its first word, top bits of its second), a version-gated u16 and a
/// base-index u16. Its corners are read as u16 values into the allocator
/// buffer, resolved to planes through callee 30 while running float
/// min/max/sum accumulators are kept, quantised to i16 fields, and used
/// with the extents to derive two clamped u16 coordinates (the corner
/// count passes through an int-double-float round trip). A second loop
/// reads dwords and merges them through callee 32 into per-cell state,
/// with callee 33 associating heap rows.
///
/// When flag bit 1 is set, a tail count and buffer are read (callee 35)
/// and 22-byte records are filled by twelve small reads each. The file is
/// closed and the object returned. Float operation order is the original's
/// throughout; min/max updates keep the old value on NaN (`comiss`+`jbe`
/// is a plain ordered `>` test for the update).
///
/// Sub-word file reads land in full-word locals: the stub always stores
/// whole words, and only the low bytes are meaningful, exactly as the
/// original's overlapping frame writes behave.
///
/// Original: 0x00d916c0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00d916c0(path: u32) -> u32 {
    unsafe {
        const MAGIC0: u8 = 0x4e; // 'N'
        const MAGIC1: u8 = 0x41; // 'A'
        const MAGIC2: u8 = 0x56; // 'V'
        const MAGIC3: u8 = 0x4d; // 'M'
        const VERSION_GATE: u32 = 0x10005;
        const KIND_MASK: u32 = 0x1e00000;
        const COUNT_SHIFT: u32 = 0x15;
        const COUNT_MASK: u32 = 0xf;
        const BASE_MASK: u32 = 0x1ffff;
        const MIN_INIT: f32 = f32::from_bits(0x46fa0000); // 32000.0
        const MAX_INIT: f32 = f32::from_bits(0xc6fa0000); // -32000.0
        const SCALE: f32 = 8.0;
        const QUANT: f32 = f32::from_bits(0x47800000); // 65536.0
        const ONE: f32 = 1.0;
        const CELL_STRIDE: u32 = 40;
        const TAIL_STRIDE: u32 = 22;
        const TLS_INDEX_GLOBAL: u32 = 0x17aba14;
        const FORCE_DWORD_FLAG: u32 = 0x179f943;
        const DBL_ADJ_TABLE: u32 = 0xfe8f50;
        const OPEN_CTX: u32 = 0x110c0a0;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(p: u32) -> u16 {
            unsafe { (p as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(p: u32) -> u8 {
            unsafe { (p as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(p: u32, v: u16) {
            unsafe { (p as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(p: u32, v: u8) {
            unsafe { (p as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { wr32(p, v.to_bits()) }
        }
        #[cfg(target_arch = "x86")]
        #[inline(always)]
        unsafe fn cvtt(x: f32) -> i32 {
            unsafe {
                core::arch::x86::_mm_cvtt_ss2si(core::arch::x86::_mm_set_ss(x))
            }
        }
        #[cfg(not(target_arch = "x86"))]
        #[inline(always)]
        unsafe fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn f64add(a: f64, b: f64) -> f64 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if path == 0 || rd8(path) == 0 {
            return 0;
        }
        let handle =
            lf_checker_rt::callee_thiscall!(1, u32, relocated(OPEN_CTX), path, relocated(0xeedcd1), 1u32, 1u32); // immediates relocated like the original's
        if handle == 0 {
            return 0;
        }
        let mut magic = [0u32; 4];
        lf_checker_rt::callee_thiscall!(2, u32, handle, (&mut magic[0] as *mut u32) as u32, 1);
        lf_checker_rt::callee_thiscall!(3, u32, handle, (&mut magic[1] as *mut u32) as u32, 1);
        lf_checker_rt::callee_thiscall!(4, u32, handle, (&mut magic[2] as *mut u32) as u32, 1);
        lf_checker_rt::callee_thiscall!(5, u32, handle, (&mut magic[3] as *mut u32) as u32, 1);
        if magic[0] as u8 != MAGIC0
            || magic[1] as u8 != MAGIC1
            || magic[2] as u8 != MAGIC2
            || magic[3] as u8 != MAGIC3
        {
            lf_checker_rt::callee_thiscall!(48, u32, handle);
            return 0;
        }
        let mut version = 0u32;
        lf_checker_rt::callee_thiscall!(6, u32, handle, (&mut version as *mut u32) as u32, 4);
        let dummy = 0u32;
        let obj = lf_checker_rt::callee_thiscall!(7, u32, (&dummy as *const u32) as u32);
        wr32(obj + 0x54, version);
        lf_checker_rt::callee_thiscall!(8, u32, handle, obj + 0x50, 4);
        lf_checker_rt::callee_thiscall!(9, u32, handle, obj + 0x80, 4);
        wr32(obj + 0x88, 0);
        let coeff = lf_checker_rt::callee_cdecl!(10, u32, obj, handle);
        wr32(obj + 0x70, coeff);
        if coeff != 0 {
            let dst = rd32(obj + 0x90);
            wr32(dst, rd32(coeff));
            wrf(dst + 4, rdf(coeff + 4));
            wrf(dst + 8, rdf(coeff + 8));
            wr32(dst + 12, rd32(coeff + 12));
        }
        wrf(obj + 0x40, fsub(rdf(coeff + 0x10), rdf(coeff)));
        wrf(obj + 0x44, fsub(rdf(coeff + 0x14), rdf(coeff + 4)));
        wrf(obj + 0x48, fsub(rdf(coeff + 0x18), rdf(coeff + 8)));
        lf_checker_rt::callee_thiscall!(11, u32, handle, obj + 0x78, 4);
        lf_checker_rt::callee_thiscall!(12, u32, handle, obj + 0x7c, 4);
        if unsafe { *lf_checker_rt::global::<u8>(FORCE_DWORD_FLAG) } != 0 {
            wr32(obj + 0x50, rd32(obj + 0x50) & 0xfffffffe);
        }
        let count_a = rd32(obj + 0x78);
        if rd32(obj + 0x50) & 1 != 0 {
            let buf_a =
                lf_checker_rt::callee_thiscall!(13, u32, (&dummy as *const u32) as u32, count_a.wrapping_mul(3));
            wr32(obj + 0x58, buf_a);
            wr32(
                obj + 0x84,
                rd32(obj + 0x84).wrapping_add(count_a.wrapping_mul(6)),
            );
            let mut i = 0u32;
            let mut off = 0u32;
            while i < count_a {
                let mut w = 0u32;
                lf_checker_rt::callee_thiscall!(14, u32, handle, (&mut w as *mut u32) as u32, 2);
                wr16(buf_a + off, w as u16);
                lf_checker_rt::callee_thiscall!(15, u32, handle, (&mut w as *mut u32) as u32, 2);
                wr16(buf_a + off + 2, w as u16);
                lf_checker_rt::callee_thiscall!(16, u32, handle, (&mut w as *mut u32) as u32, 2);
                wr16(buf_a + off + 4, w as u16);
                off += 6;
                i += 1;
            }
        } else {
            let buf_b =
                lf_checker_rt::callee_thiscall!(17, u32, (&dummy as *const u32) as u32, count_a);
            wr32(obj + 0x5c, buf_b);
            wr32(
                obj + 0x84,
                rd32(obj + 0x84).wrapping_add(count_a.wrapping_mul(16)),
            );
            let mut i = 0u32;
            let mut off = 0u32;
            while i < count_a {
                lf_checker_rt::callee_thiscall!(18, u32, handle, buf_b + off, 4);
                lf_checker_rt::callee_thiscall!(19, u32, handle, buf_b + off + 4, 4);
                lf_checker_rt::callee_thiscall!(20, u32, handle, buf_b + off + 8, 4);
                off += 0x10;
                i += 1;
            }
        }
        lf_checker_rt::callee_thiscall!(21, u32, handle, obj + 0x68, 4);
        let count_c = rd32(obj + 0x68);
        // edi is reloaded with the file handle here (saved slot); every later read and the close use it.
        // Thread-local allocator: the slot index comes from a global, the
        // chain runs slot -> +0x10 -> vtable -> slot 8, called as thiscall
        // with (size, 0x10, 1) and ecx at the +0x10 object.
        let tls_idx = unsafe { *lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL) };
        let tls_obj = lf_checker_rt::tls_slot(tls_idx as usize);
        let heap_mgr = rd32(tls_obj + 0x10);
        let vtable = rd32(heap_mgr);
        let alloc_addr = rd32(vtable + 8);
        let alloc_fn: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(alloc_addr as usize) };
        let tls_buf = unsafe {
            alloc_fn(
                heap_mgr,
                count_c.wrapping_mul(2).wrapping_add(0x10),
                0x10,
                1,
            )
        };
        wr32(obj + 0x60, tls_buf);
        wr32(
            obj + 0x84,
            rd32(obj + 0x84).wrapping_add(count_c.wrapping_mul(2)),
        );
        let corner_buf = lf_checker_rt::callee_thiscall!(
            23,
            u32,
            (&dummy as *const u32) as u32,
            count_c
        );
        wr32(obj + 0x64, corner_buf);
        wr32(
            obj + 0x84,
            rd32(obj + 0x84).wrapping_add(count_c.wrapping_mul(8)),
        );
        let count_b = rd32(obj + 0x7c);
        // Callee 24's out-words land 0x24 bytes past its object pointer.
        let mut ctor24 = [0u32; 12];
        let cells = lf_checker_rt::callee_thiscall!(
            24,
            u32,
            ctor24.as_mut_ptr() as u32,
            count_b
        );
        wr32(obj + 0x6c, cells);
        wr32(
            obj + 0x84,
            rd32(obj + 0x84).wrapping_add(count_b.wrapping_mul(40)),
        );
        let spare = rd32(obj + 0x80);
        // Per-cell state words seeded by callee 24's out-words.
        let mut st34 = ctor24[9];
        let mut st38 = ctor24[10];
        // Exact mirrors of the frame words: e88 is written by the merge
        // stub and the per-iter mask only, e8c additionally by the merge.
        let mut e88 = 0u32;
        let mut e8c = 0u32;
        let mut slot30 = [0u32; 3];
        let mut w32 = [0u32; 6];
        let mut i = 0u32;
        while i < count_b {
            let rec = cells.wrapping_add(i.wrapping_mul(CELL_STRIDE));
            let mut b0w = 0u32;
            lf_checker_rt::callee_thiscall!(
                25, u32, handle, (&mut b0w as *mut u32) as u32, 1
            );
            let b0 = b0w as u8;
            let w0 = rd32(rec);
            wr32(rec, w0 ^ ((b0 as u32 ^ w0) & 0x1fffff));
            let mut b1w = 0u32;
            lf_checker_rt::callee_thiscall!(
                26, u32, handle, (&mut b1w as *mut u32) as u32, 1
            );
            let b1 = b1w as u8;
            let w0b = rd32(rec);
            wr32(
                rec,
                w0b ^ (((b1 as u32) << COUNT_SHIFT ^ w0b) & KIND_MASK),
            );
            let w1 = rd32(rec + 4);
            wr32(
                rec + 4,
                (((b1 >> 4) as u32) << 0x1d) | (w1 & 0x1fffffff),
            );
            if version < VERSION_GATE {
                wr16(rec + 0x1c, rd16(rec + 0x1c) & 0xfff0);
            } else {
                let mut v16 = 0u32;
                lf_checker_rt::callee_thiscall!(
                    27, u32, handle, (&mut v16 as *mut u32) as u32, 2
                );
                wr16(rec + 0x1c, v16 as u16);
            }
            let mut bw = 0u32;
            lf_checker_rt::callee_thiscall!(
                28, u32, handle, (&mut bw as *mut u32) as u32, 2
            );
            let w1b = rd32(rec + 4);
            wr32(rec + 4, w1b ^ ((bw ^ w1b) & BASE_MASK));
            st34 |= 0xffff0fff;
            st34 &= 0xffff0fff;
            e88 = st34;
            st38 |= 0x0fffffff;
            st38 &= 0xefffffff;
            e8c = st38;
            let w0c = rd32(rec);
            let count2 = (w0c >> COUNT_SHIFT) & COUNT_MASK;
            let base = rd32(rec + 4) & BASE_MASK;
            let mut sum0 = 0.0f32;
            let mut sum1 = 0.0f32;
            let mut min0 = MIN_INIT;
            let mut max0 = MAX_INIT;
            let mut min1 = MIN_INIT;
            let mut max1 = MAX_INIT;
            let mut min2 = MIN_INIT;
            let mut max2 = MAX_INIT;
            if w0c & KIND_MASK != 0 {
                let mut k = 0u32;
                while k < count2 {
                    let mut uw = 0u32;
                    lf_checker_rt::callee_thiscall!(
                        29, u32, handle, (&mut uw as *mut u32) as u32, 2
                    );
                    let u = uw as u16;
                    wr16(tls_buf + (base + k) * 2, u);
                    lf_checker_rt::callee_thiscall!(
                        30, u32, obj, u as u32, slot30.as_mut_ptr() as u32
                    );
                    let s0 = f32::from_bits(slot30[0]);
                    let s1 = f32::from_bits(slot30[1]);
                    let s2 = f32::from_bits(slot30[2]);
                    sum0 = fadd(sum0, s0);
                    sum1 = fadd(sum1, s1);
                    if min0 > s0 {
                        min0 = s0;
                    }
                    if s0 > max0 {
                        max0 = s0;
                    }
                    if min1 > s1 {
                        min1 = s1;
                    }
                    if s1 > max1 {
                        max1 = s1;
                    }
                    if min2 > s2 {
                        min2 = s2;
                    }
                    if s2 > max2 {
                        max2 = s2;
                    }
                    k += 1;
                }
            }
            wr16(rec + 0x10, cvtt(fmul(min0, SCALE)) as u16);
            wr16(rec + 0x14, cvtt(fmul(min1, SCALE)) as u16);
            wr16(rec + 0x18, cvtt(fmul(min2, SCALE)) as u16);
            wr16(rec + 0x12, cvtt(fmul(max0, SCALE)) as u16);
            wr16(rec + 0x16, cvtt(fmul(max1, SCALE)) as u16);
            wr16(rec + 0x1a, cvtt(fmul(max2, SCALE)) as u16);
            // Corner count through int -> double -> float with the table
            // add. The table index is count2 >> 31, always 0 (count2 <= 0xf).
            let adj =
                f64::from_bits(unsafe { *lf_checker_rt::global::<u64>(DBL_ADJ_TABLE) });
            let d = f64add(count2 as f64, adj);
            let inv = fdiv(ONE, d as f32);
            let n0 = fmul(sum0, inv);
            let n1 = fmul(sum1, inv);
            let q0 = fmul(
                fdiv(fsub(n0, rdf(coeff)), rdf(obj + 0x40)),
                QUANT,
            );
            let c0 = cvtt(q0);
            let c0c: u32 = if c0 < 0 {
                0
            } else if (c0 as u32) > 0xffff {
                0xffff
            } else {
                c0 as u32
            };
            wr16(rec + 0x1e, c0c as u16);
            let q1 = fmul(
                fdiv(fsub(n1, rdf(coeff + 4)), rdf(obj + 0x44)),
                QUANT,
            );
            let c1 = cvtt(q1);
            let c1c: u32 = if c1 < 0 {
                0
            } else if (c1 as u32) > 0xffff {
                0xffff
            } else {
                c1 as u32
            };
            wr16(rec + 0x20, c1c as u16);
            wr8(rec + 0x23, 0);
            wr32(rec + 0x24, 0);
            let mut k = 0u32;
            while k < count2 {
                lf_checker_rt::callee_thiscall!(
                    31, u32, handle, (&mut w32[0] as *mut u32) as u32, 4
                );
                w32[3] = e88;
                w32[4] = e8c;
                lf_checker_rt::callee_thiscall!(
                    32, u32, (&mut w32[0] as *mut u32) as u32,
                    (&mut w32[3] as *mut u32) as u32
                );
                let new_lo = w32[3];
                let new_hi = w32[4];
                e88 = new_lo;
                e8c = new_hi;
                // Nibble merge, high word as the base like the original.
                let mut m = new_hi ^ new_lo;
                m &= 0xfff;
                let mut mc = new_hi ^ m;
                m = (new_lo >> 4) ^ mc;
                m &= 0xffff000;
                mc ^= m;
                st38 = mc;
                st34 = new_lo;
                e8c = mc;
                w32[4] = mc;
                let row = rd32(obj + 0x64)
                    .wrapping_add((base + k).wrapping_mul(8));
                lf_checker_rt::callee_thiscall!(
                    33, u32, (&mut w32[3] as *mut u32) as u32, row
                );
                k += 1;
            }
            wr32(
                rec + 4,
                rd32(rec + 4) ^ (spare.wrapping_shl(0x11) & 0x1ffe0000),
            );
            wr32(rec + 8, 0);
            wr16(rec + 0x1c, rd16(rec + 0x1c) & 0xffef);
            i += 1;
        }
        if rd32(obj + 0x50) & 2 != 0 {
            lf_checker_rt::callee_thiscall!(34, u32, handle, obj + 0x8c, 4);
            let count_t = rd32(obj + 0x8c);
            let tbuf = lf_checker_rt::callee_thiscall!(
                35,
                u32,
                (&dummy as *const u32) as u32,
                count_t
            );
            wr32(obj + 0x74, tbuf);
            let mut t = 0u32;
            while t < count_t {
                let r = tbuf.wrapping_add(t.wrapping_mul(TAIL_STRIDE));
                lf_checker_rt::callee_thiscall!(36, u32, handle, r + 0x14, 1);
                lf_checker_rt::callee_thiscall!(37, u32, handle, r + 0x15, 1);
                lf_checker_rt::callee_thiscall!(38, u32, handle, r + 6, 2);
                lf_checker_rt::callee_thiscall!(39, u32, handle, r + 8, 2);
                lf_checker_rt::callee_thiscall!(40, u32, handle, r, 2);
                lf_checker_rt::callee_thiscall!(41, u32, handle, r + 2, 2);
                lf_checker_rt::callee_thiscall!(42, u32, handle, r + 4, 2);
                lf_checker_rt::callee_thiscall!(43, u32, handle, r + 0x10, 2);
                lf_checker_rt::callee_thiscall!(44, u32, handle, r + 0x12, 2);
                lf_checker_rt::callee_thiscall!(45, u32, handle, r + 0xa, 2);
                lf_checker_rt::callee_thiscall!(46, u32, handle, r + 0xc, 2);
                lf_checker_rt::callee_thiscall!(47, u32, handle, r + 0xe, 2);
                t += 1;
            }
        }
        lf_checker_rt::callee_thiscall!(48, u32, handle);
        obj
    }
});
