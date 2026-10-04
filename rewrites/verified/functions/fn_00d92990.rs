// original: 0x00D92990 tri_area_flag (proposed)

/// Flag one entry of a 40-byte-entry table by the area of the triangle its
/// three resolved points form.
///
/// `this` points to an object holding two table pointers: the entry array at
/// `+0x6c` and a 16-bit index table at `+0x60`. Entry `idx` is 40 bytes wide:
/// a flag word at `+0` and a packed word at `+4`. The low two flag bits are
/// cleared, then each of three point ids is read as a 16-bit word from the
/// index table at `(packed & 0x1ffff) * 2 + {0, 2, 4}` and resolved to a
/// three-float point through the point callee (thiscall, id, out-pointer).
///
/// The area comes from Heron's formula in doubled form: with edge lengths a,
/// b, c and perimeter S, area = sqrt(S * (S - 2a) * (S - 2b) * (S - 2c)) / 4.
/// Bit 0 of the flag word is set when `t1 > area`; otherwise, when the first
/// test does not fire, bit 2 (value 2) is set when `area > t2`. Both
/// comparisons are strict: an unordered (NaN) result sets neither bit, and a
/// negative product under the square root yields NaN the same way.
///
/// Edge cases: `idx * 40` wraps modulo 2^32; a wild index faults on the flag
/// access before any call. A packed key past the index table faults on the
/// word read. The entry word is re-read from memory for every point id, and
/// the packed word is re-read too (the callee only writes its out-pointer,
/// so it cannot change).
///
/// Original: 0x00D92990 (thiscall, three stack words: index, t1 bits, t2
/// bits; no defined return value, eax is leftover).
lf_checker_rt::export!(thiscall, rw_00D92990(this: u32, idx: u32, t1: u32, t2: u32) -> u32 {
    unsafe {
        const OBJ_U16TABLE: u32 = 0x60;
        const OBJ_ENTRIES: u32 = 0x6c;
        const ENTRY_PACKED: u32 = 0x04;
        const KEY_MASK: u32 = 0x1ffff;
        const FLAG_KEEP: u32 = 0xfffffffc;
        const TWO: f32 = 2.0;
        const QUARTER: f32 = 0.25;
        const CALLEE_P: u32 = 1;
        const CALLEE_Q: u32 = 2;
        const CALLEE_R: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let entries = rd32(this.wrapping_add(OBJ_ENTRIES));
        let entry = entries.wrapping_add(idx.wrapping_mul(5).wrapping_mul(8));
        wr32(entry, rd32(entry) & FLAG_KEEP);
        let table = rd32(this.wrapping_add(OBJ_U16TABLE));
        let mut p = [0.0f32; 3];
        let mut q = [0.0f32; 3];
        let mut r = [0.0f32; 3];
        let packed = rd32(entry.wrapping_add(ENTRY_PACKED));
        let key = (packed & KEY_MASK).wrapping_mul(2);
        let v0 = rd16(table.wrapping_add(key));
        lf_checker_rt::callee_thiscall!(CALLEE_P, u32, this, v0, p.as_mut_ptr() as u32);
        let packed = rd32(entry.wrapping_add(ENTRY_PACKED));
        let key = (packed & KEY_MASK).wrapping_mul(2);
        let v1 = rd16(table.wrapping_add(key).wrapping_add(2));
        lf_checker_rt::callee_thiscall!(CALLEE_Q, u32, this, v1, q.as_mut_ptr() as u32);
        let packed = rd32(entry.wrapping_add(ENTRY_PACKED));
        let key = (packed & KEY_MASK).wrapping_mul(2);
        let v2 = rd16(table.wrapping_add(key).wrapping_add(4));
        lf_checker_rt::callee_thiscall!(CALLEE_R, u32, this, v2, r.as_mut_ptr() as u32);

        // Edge lengths: a = |P - R|, b = |Q - P|, c = |R - Q|, each as the
        // original orders its squares and adds.
        let dx0 = sub(p[0], r[0]);
        let dy0 = sub(p[1], r[1]);
        let qxpx = sub(q[0], p[0]);
        let qypy = sub(q[1], p[1]);
        let qzpz = sub(q[2], p[2]);
        let pzrz = sub(p[2], r[2]);
        let rxqx = sub(r[0], q[0]);
        let ryqy = sub(r[1], q[1]);
        let rzqz = sub(r[2], q[2]);
        let b2 = add(add(mul(qypy, qypy), mul(qxpx, qxpx)), mul(qzpz, qzpz));
        let a2 = add(add(mul(dy0, dy0), mul(dx0, dx0)), mul(pzrz, pzrz));
        let b = b2.sqrt();
        let a = a2.sqrt();
        let c2 = add(add(mul(ryqy, ryqy), mul(rxqx, rxqx)), mul(rzqz, rzqz));
        let c = c2.sqrt();
        let s = add(add(b, a), c);
        let a2x = mul(a, TWO);
        let b2x = mul(b, TWO);
        let c2x = mul(c, TWO);
        let prod = mul(mul(mul(sub(s, a2x), s), sub(s, b2x)), sub(s, c2x));
        let area = mul(prod.sqrt(), QUARTER);

        // Strict comparisons: NaN on either side sets nothing. The second
        // test runs only when the first does not fire (early return).
        let t1f = f32::from_bits(t1);
        let t2f = f32::from_bits(t2);
        if t1f > area {
            wr32(entry, rd32(entry) | 1);
        } else if area > t2f {
            wr32(entry, rd32(entry) | 2);
        }
        0
    }
});
