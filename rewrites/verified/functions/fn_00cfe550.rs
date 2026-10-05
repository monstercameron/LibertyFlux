// original: 0x00cfe550 ped_task_aim_check
/// Decide an aim gate: set or clear bit 1 of the state byte at `this+0x35`.
///
/// `this` points at the seek record and `obj` at a task object holding a
/// driver pointer at `+0xd68` and a flag byte at `+0x35` whose bit 1 this
/// function sets or clears. `flag` (low byte of the second stack word)
/// selects a mode pre-check.
///
/// When `flag` is set, bits 5-6 of the driver's first word decide: mode 1
/// sets the bit and returns 1, mode 0 clears the bit and returns 0, other
/// modes continue. Otherwise the 2D seek direction (mover minus owner
/// position, normalised, z 0) feeds three calls: the driver call (thiscall,
/// two frame buffers: a 3-word direction input and a 3-word output), then
/// two double-returning calls whose results scale and combine the driver's
/// output into two factors. The dot product of a vector (the direction, or
/// the owner's position block at `+0x10` when byte `+0x218` is clear and
/// `+0x219` set) with those factors decides: positive clears the bit, zero
/// or negative (or NaN) sets it. The float operation order is the original's.
/// The normalising constant is read from the relocated image.
///
/// Limits: the two double calls take an 8-byte XMM0 argument the checker
/// cannot transport (4 bytes only), so their argument is unlogged; their
/// double results arrive in XMM0, which Rust cannot receive, so the contract
/// scripts doubles with zero high word and the rewrite rebuilds them from
/// the low word the stub also places in EAX. The return value is uncompared
/// (a frame address on one path). The driver-null exit returns the entry EAX
/// and is excluded by the contract.
///
/// Original: thiscall, ECX plus two stack words.
lf_checker_rt::export!(thiscall, rw_00cfe550(this: u32, obj: u32, flag: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const DRV: u32 = 0xd68;
        const FLAGB: u32 = 0x35;
        const ONE_VA: u32 = 0x00fe88e8;
        const GATE: u32 = 1;
        const K2: u32 = 2;
        const K3: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn flagbit(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn set_flagbit(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let one = f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
        let d68 = rd32(obj + DRV);
        if d68 == 0 {
            return 0; // unreachable: pinned nonzero (original returns entry EAX)
        }
        let (sx, sy) = {
            let s = rd32(this + 0x30);
            if s == 0 {
                (rdf(this + 0x20), rdf(this + 0x24))
            } else {
                let t = rd32(s + POS);
                (rdf(t + PX), rdf(t + PY))
            }
        };
        if flag & 0xff != 0 {
            let mode = (rd32(d68) >> 5) & 3;
            if mode == 1 {
                set_flagbit(this + FLAGB, flagbit(this + FLAGB) | 2);
                return 1;
            }
            if mode == 0 {
                set_flagbit(this + FLAGB, flagbit(this + FLAGB) & 0xfd);
                return 0;
            }
        }
        let epos = rd32(obj + POS);
        let dy = sub(sy, rdf(epos + PY));
        let dx = sub(sx, rdf(epos + PX));
        let len2 = add(mul(dx, dx), mul(dy, dy));
        let inv = if len2 <= 0.0 {
            0.0
        } else {
            core::hint::black_box(one) / core::hint::black_box(core::hint::black_box(len2).sqrt())
        };
        let vx = mul(dx, inv);
        let vy = mul(inv, dy);
        let vz = mul(inv, 0.0);
        let mut buf_a = [vx.to_bits(), vy.to_bits(), vz.to_bits()];
        let mut buf_b = [0u32; 3];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            GATE, u32, d68, buf_b.as_mut_ptr() as u32, buf_a.as_ptr() as u32);
        // Double results arrive in XMM0 (unreceivable); the stub also leaves
        // the low word in EAX, and the contract scripts zero-high doubles.
        let r2lo: u32 = lf_checker_rt::callee_thiscall!(K2, u32, d68);
        let f2 = f64::from_bits(r2lo as u64) as f32;
        let r3lo: u32 = lf_checker_rt::callee_thiscall!(K3, u32, d68);
        let f3 = f64::from_bits(r3lo as u64) as f32;
        let w0 = f32::from_bits(buf_b[0]);
        let w1 = f32::from_bits(buf_b[1]);
        let w2 = f32::from_bits(buf_b[2]);
        let r5 = sub(mul(w0, f3), mul(w1, f2));
        let r4 = add(mul(w1, f3), mul(w0, f2));
        let (hx, hy, hz) = if rd32(obj + 0x218) & 0xff != 0
            || rd32(obj + 0x219) & 0xff == 0
        {
            (vx, vy, vz)
        } else {
            let b = rd32(epos + 0x10);
            (f32::from_bits(b), f32::from_bits(rd32(epos + 0x14)), f32::from_bits(rd32(epos + 0x18)))
        };
        let dot = add(add(mul(hy, r4), mul(hx, r5)), mul(hz, w2));
        if dot > 0.0 {
            set_flagbit(this + FLAGB, flagbit(this + FLAGB) & 0xfd);
        } else {
            set_flagbit(this + FLAGB, flagbit(this + FLAGB) | 2);
        }
        1
    }
});

/// Wrong version of rw_00cfe550: the normalising constant plus 1.0.
lf_checker_rt::export!(thiscall, mut_00cfe550(this: u32, obj: u32, flag: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const DRV: u32 = 0xd68;
        const FLAGB: u32 = 0x35;
        const ONE_VA: u32 = 0x00fe88e8;
        const GATE: u32 = 1;
        const K2: u32 = 2;
        const K3: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn flagbit(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn set_flagbit(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        // MUTANT: constant plus 1.0 instead of the constant.
        let one = f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned()) + 1.0;
        let d68 = rd32(obj + DRV);
        if d68 == 0 {
            return 0; // unreachable: pinned nonzero (original returns entry EAX)
        }
        let (sx, sy) = {
            let s = rd32(this + 0x30);
            if s == 0 {
                (rdf(this + 0x20), rdf(this + 0x24))
            } else {
                let t = rd32(s + POS);
                (rdf(t + PX), rdf(t + PY))
            }
        };
        if flag & 0xff != 0 {
            let mode = (rd32(d68) >> 5) & 3;
            if mode == 1 {
                set_flagbit(this + FLAGB, flagbit(this + FLAGB) | 2);
                return 1;
            }
            if mode == 0 {
                set_flagbit(this + FLAGB, flagbit(this + FLAGB) & 0xfd);
                return 0;
            }
        }
        let epos = rd32(obj + POS);
        let dy = sub(sy, rdf(epos + PY));
        let dx = sub(sx, rdf(epos + PX));
        let len2 = add(mul(dx, dx), mul(dy, dy));
        let inv = if len2 <= 0.0 {
            0.0
        } else {
            core::hint::black_box(one) / core::hint::black_box(core::hint::black_box(len2).sqrt())
        };
        let vx = mul(dx, inv);
        let vy = mul(inv, dy);
        let vz = mul(inv, 0.0);
        let mut buf_a = [vx.to_bits(), vy.to_bits(), vz.to_bits()];
        let mut buf_b = [0u32; 3];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            GATE, u32, d68, buf_b.as_mut_ptr() as u32, buf_a.as_ptr() as u32);
        // Double results arrive in XMM0 (unreceivable); the stub also leaves
        // the low word in EAX, and the contract scripts zero-high doubles.
        let r2lo: u32 = lf_checker_rt::callee_thiscall!(K2, u32, d68);
        let f2 = f64::from_bits(r2lo as u64) as f32;
        let r3lo: u32 = lf_checker_rt::callee_thiscall!(K3, u32, d68);
        let f3 = f64::from_bits(r3lo as u64) as f32;
        let w0 = f32::from_bits(buf_b[0]);
        let w1 = f32::from_bits(buf_b[1]);
        let w2 = f32::from_bits(buf_b[2]);
        let r5 = sub(mul(w0, f3), mul(w1, f2));
        let r4 = add(mul(w1, f3), mul(w0, f2));
        let (hx, hy, hz) = if rd32(obj + 0x218) & 0xff != 0
            || rd32(obj + 0x219) & 0xff == 0
        {
            (vx, vy, vz)
        } else {
            let b = rd32(epos + 0x10);
            (f32::from_bits(b), f32::from_bits(rd32(epos + 0x14)), f32::from_bits(rd32(epos + 0x18)))
        };
        let dot = add(add(mul(hy, r4), mul(hx, r5)), mul(hz, w2));
        if dot > 0.0 {
            set_flagbit(this + FLAGB, flagbit(this + FLAGB) & 0xfd);
        } else {
            set_flagbit(this + FLAGB, flagbit(this + FLAGB) | 2);
        }
        1
    }
});
