// original: 0x00a1ed80 ped_task_remap_vectors (proposed)

/// Remap three vectors of a task object into its owner's frame, then offer
/// the owner to four slot tables.
///
/// `this` points to a task record, `arg` to a frame record holding a 3x3
/// matrix and an offset vector. The rewrite first forms the difference `d`
/// of the anchor points (`this+0x420..0x42c` minus `arg+0x30..0x3c`) and
/// stores three dot products at `this+0x430..0x43c`: row 0 of the matrix
/// (`arg+0..0xc`) dotted with `d`, then row 1 (`arg+0x10..0x1c`), then row 2
/// (`arg+0x20..0x2c`). The float operation order below is the original's.
///
/// It then calls the refresh callee (id 1, thiscall on `this`, no stack
/// arguments) and walks four slot tables. Table `k` has a signed count at
/// `this+0x400+4*k`, one flag byte per slot at `this+0x410+k` plus the slot
/// index, slot objects starting at a base with a fixed stride, and its own
/// callee (ids 2..5, thiscall on the slot object, one stack argument holding
/// `arg`):
///
/// | table | base  | stride | flags | callee |
/// |-------|-------|--------|-------|--------|
/// | 0     | +0x90 | 0xb0   | +0x410| id 2   |
/// | 1     | +0x140| 0xc0   | +0x411| id 3   |
/// | 2     | +0x200| 0xe0   | +0x412| id 4   |
/// | 3     | +0x2e0| 0x120  | +0x413| id 5   |
///
/// A slot whose flag byte is zero is skipped. The count is re-read from
/// memory every iteration and a count of zero or less skips the table.
/// Counts are caller-bounded small in practice; the contract pins them.
///
/// The returned value in eax is whatever the last loop left there: the final
/// index of table 0, then each later table's running object pointer after
/// its last slot (or the carried value when the table is skipped or makes
/// no calls). Callers treat the function as returning nothing meaningful;
/// the value is reproduced exactly so the comparison stays strict.
///
/// Original: 0x00a1ed80 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1ed80(this: u32, arg: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x420;
        const OUT0: u32 = 0x430;
        const COUNT0: u32 = 0x400;
        const FLAG0: u32 = 0x410;
        const BASES: [u32; 4] = [0x90, 0x140, 0x200, 0x2e0];
        const STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];

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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        // Anchor difference d = this.anchor - arg.offset.
        let dx = sub(rdf(this + ANCHOR), rdf(arg + 0x30));
        let dy = sub(rdf(this + ANCHOR + 4), rdf(arg + 0x34));
        let dz = sub(rdf(this + ANCHOR + 8), rdf(arg + 0x38));
        // Row 0: (m01*dy + m00*dx) + m02*dz.
        let t1 = mul(rdf(arg + 4), dy);
        let t0 = mul(rdf(arg), dx);
        let s = add(t1, t0);
        let t0b = mul(rdf(arg + 8), dz);
        wrf(this + OUT0, add(s, t0b));
        // Row 1: (m11*dy + dx*m10) + m12*dz.
        let c = mul(dx, rdf(arg + 0x10));
        let t = mul(rdf(arg + 0x14), dy);
        let t = add(t, c);
        let u = mul(rdf(arg + 0x18), dz);
        wrf(this + OUT0 + 4, add(t, u));
        // Row 2: (m21*dy + dx*m20) + m22*dz.
        let t = rdf(arg + 0x24);
        let dx2 = mul(dx, rdf(arg + 0x20));
        let u = rdf(arg + 0x28);
        let t = mul(t, dy);
        let u = mul(u, dz);
        let t = add(t, dx2);
        wrf(this + OUT0 + 8, add(t, u));

        // Refresh, then the four slot tables.
        let mut eax: u32;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        eax = 0;
        // Table 0: eax tracks the slot index.
        if (rd32(this + COUNT0) as i32) > 0 {
            let mut idx: u32 = 0;
            let mut obj: u32 = this.wrapping_add(BASES[0]);
            loop {
                if rd8(this.wrapping_add(idx).wrapping_add(FLAG0)) != 0 {
                    lf_checker_rt::callee_thiscall!(2, u32, obj, arg);
                    eax = idx;
                }
                idx = idx.wrapping_add(1);
                eax = eax.wrapping_add(1);
                obj = obj.wrapping_add(STRIDES[0]);
                if !((idx as i32) < rd32(this + COUNT0) as i32) {
                    break;
                }
            }
        }
        // Tables 1..3: eax tracks the running object pointer.
        let mut k: usize = 1;
        while k < 4 {
            let count = COUNT0 + (k as u32) * 4;
            let flags = FLAG0 + k as u32;
            if (rd32(this + count) as i32) > 0 {
                let mut idx: u32 = 0;
                let mut obj: u32 = this.wrapping_add(BASES[k]);
                eax = obj;
                loop {
                    if rd8(this.wrapping_add(idx).wrapping_add(flags)) != 0 {
                        let id: u32 = (k as u32) + 2;
                        match id {
                            3 => {
                                lf_checker_rt::callee_thiscall!(3, u32, obj, arg);
                            }
                            4 => {
                                lf_checker_rt::callee_thiscall!(4, u32, obj, arg);
                            }
                            _ => {
                                lf_checker_rt::callee_thiscall!(5, u32, obj, arg);
                            }
                        }
                        eax = obj;
                    }
                    idx = idx.wrapping_add(1);
                    eax = eax.wrapping_add(STRIDES[k]);
                    obj = obj.wrapping_add(STRIDES[k]);
                    if !((idx as i32) < rd32(this + count) as i32) {
                        break;
                    }
                }
            }
            k += 1;
        }
        eax
    }
});
