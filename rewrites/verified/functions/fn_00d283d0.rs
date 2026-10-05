// original: 0x00d283d0 target_heading_update
/// Refresh the heading at `this+0x10` from a direction sample or a fallback.
///
/// Reads the ped at `this+0x14` and its kind tag `([ped+0x28] >> 6) & 0xF`.
/// For tags 2-4, samples a 3-word direction through vtable slot `+0xec` of
/// the ped; when its squared length exceeds a threshold from the image, the
/// direction is normalised (by 1/sqrt, or 0.0 for an exactly zero length),
/// the x lane sign-flipped, both lanes widened to doubles, and passed to an
/// angle helper whose double answer is narrowed back to float and stored to
/// `this+0x10`. Any other tag, a short sample, or a null `[ped+0x20]` takes
/// the fallback: a live `[ped+0x20]` block feeds the same angle helper from
/// its `+0x10`/`+0x14` lanes, while a null one copies `[ped+0x1c]` to
/// `this+0x10` verbatim. Returns the helper's low answer word on the call
/// paths and 0 on the null path.
///
/// The helper's double arguments travel in XMM0/XMM1, which the checker
/// cannot transport, so they are unlogged; its double answer is constrained
/// to zero-high-word scripts and rebuilt from EAX (see narrowed).
///
/// Original: thiscall, ECX only.
lf_checker_rt::export!(thiscall, rw_00d283d0(this: u32) -> u32 {
    unsafe {
        const SAMPLE: u32 = 1;
        const ANGLE: u32 = 2;
        const THRESH_VA: u32 = 0x00fe86d4;
        const ONE_VA: u32 = 0x00fe88e8;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        const VT_SAMPLE: u32 = 0xec;
        let ped = ((this + 0x14) as *const u32).read_unaligned();
        let t = ((((ped + 0x28) as *const u32).read_unaligned() >> 6) & 0xF) as u32;
        if t > 1 && t < 5 {
            let vt = (ped as *const u32).read_unaligned();
            let slot = ((vt + VT_SAMPLE) as *const u32).read_unaligned();
            let sample: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let mut buf = [0u32; 3];
            let _ = sample(ped, buf.as_mut_ptr() as u32);
            let x = f32::from_bits(buf[0]);
            let y = f32::from_bits(buf[1]);
            let z = f32::from_bits(buf[2]);
            let yy = core::hint::black_box(y) * core::hint::black_box(y);
            let xx = core::hint::black_box(x) * core::hint::black_box(x);
            let mut len2 = core::hint::black_box(xx) + core::hint::black_box(yy);
            let zz = core::hint::black_box(z) * core::hint::black_box(z);
            len2 = core::hint::black_box(len2) + core::hint::black_box(zz);
            let thresh =
                f32::from_bits(lf_checker_rt::global::<u32>(THRESH_VA).read_unaligned());
            if len2 > thresh {
                let inv = if len2 == 0.0 {
                    0.0
                } else {
                    let one = f32::from_bits(
                        lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
                    core::hint::black_box(one)
                        / core::hint::black_box(core::hint::black_box(len2).sqrt())
                };
                let vx = core::hint::black_box(inv) * core::hint::black_box(x);
                let vy = core::hint::black_box(inv) * core::hint::black_box(y);
                let mask =
                    lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
                // Sign-flipped vx feeds the helper's unlogged XMM0 argument.
                let _nx = f32::from_bits(vx.to_bits() ^ mask);
                let _ = _nx;
                let _ = vy;
                let rlo: u32 = lf_checker_rt::callee_stdcall!(ANGLE, u32,);
                let d = f64::from_bits(rlo as u64);
                ((this + 0x10) as *mut u32).write_unaligned((d as f32).to_bits());
                return rlo;
            }
        }
        let ped2 = ((this + 0x14) as *const u32).read_unaligned();
        let m = ((ped2 + 0x20) as *const u32).read_unaligned();
        if m == 0 {
            let f = ((ped2 + 0x1c) as *const u32).read_unaligned();
            ((this + 0x10) as *mut u32).write_unaligned(f);
            return 0;
        }
        let mx = f32::from_bits(((m + 0x10) as *const u32).read_unaligned());
        let my = f32::from_bits(((m + 0x14) as *const u32).read_unaligned());
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        // Sign-flipped mx feeds the helper's unlogged XMM0 argument.
        let _nmx = f32::from_bits(mx.to_bits() ^ mask);
        let _ = _nmx;
        let _ = my;
        let rlo2: u32 = lf_checker_rt::callee_stdcall!(ANGLE, u32,);
        let d2 = f64::from_bits(rlo2 as u64);
        ((this + 0x10) as *mut u32).write_unaligned((d2 as f32).to_bits());
        rlo2
    }
});

/// Wrong version of rw_00d283d0: the null path stores 0 instead of the copy.
lf_checker_rt::export!(thiscall, mut_00d283d0(this: u32) -> u32 {
    unsafe {
        const SAMPLE: u32 = 1;
        const ANGLE: u32 = 2;
        const THRESH_VA: u32 = 0x00fe86d4;
        const ONE_VA: u32 = 0x00fe88e8;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        const VT_SAMPLE: u32 = 0xec;
        let ped = ((this + 0x14) as *const u32).read_unaligned();
        let t = ((((ped + 0x28) as *const u32).read_unaligned() >> 6) & 0xF) as u32;
        if t > 1 && t < 5 {
            let vt = (ped as *const u32).read_unaligned();
            let slot = ((vt + VT_SAMPLE) as *const u32).read_unaligned();
            let sample: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let mut buf = [0u32; 3];
            let _ = sample(ped, buf.as_mut_ptr() as u32);
            let x = f32::from_bits(buf[0]);
            let y = f32::from_bits(buf[1]);
            let z = f32::from_bits(buf[2]);
            let yy = core::hint::black_box(y) * core::hint::black_box(y);
            let xx = core::hint::black_box(x) * core::hint::black_box(x);
            let mut len2 = core::hint::black_box(xx) + core::hint::black_box(yy);
            let zz = core::hint::black_box(z) * core::hint::black_box(z);
            len2 = core::hint::black_box(len2) + core::hint::black_box(zz);
            let thresh =
                f32::from_bits(lf_checker_rt::global::<u32>(THRESH_VA).read_unaligned());
            if len2 > thresh {
                let inv = if len2 == 0.0 {
                    0.0
                } else {
                    let one = f32::from_bits(
                        lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
                    core::hint::black_box(one)
                        / core::hint::black_box(core::hint::black_box(len2).sqrt())
                };
                let vx = core::hint::black_box(inv) * core::hint::black_box(x);
                let vy = core::hint::black_box(inv) * core::hint::black_box(y);
                let mask =
                    lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
                let _nx = f32::from_bits(vx.to_bits() ^ mask);
                let _ = _nx;
                let _ = vy;
                let rlo: u32 = lf_checker_rt::callee_stdcall!(ANGLE, u32,);
                let d = f64::from_bits(rlo as u64);
                ((this + 0x10) as *mut u32).write_unaligned((d as f32).to_bits());
                return rlo;
            }
        }
        let ped2 = ((this + 0x14) as *const u32).read_unaligned();
        let m = ((ped2 + 0x20) as *const u32).read_unaligned();
        if m == 0 {
            // MUTANT: 0 instead of the [ped+0x1c] copy.
            ((this + 0x10) as *mut u32).write_unaligned(0);
            return 0;
        }
        let mx = f32::from_bits(((m + 0x10) as *const u32).read_unaligned());
        let my = f32::from_bits(((m + 0x14) as *const u32).read_unaligned());
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        let _nmx = f32::from_bits(mx.to_bits() ^ mask);
        let _ = _nmx;
        let _ = my;
        let rlo2: u32 = lf_checker_rt::callee_stdcall!(ANGLE, u32,);
        let d2 = f64::from_bits(rlo2 as u64);
        ((this + 0x10) as *mut u32).write_unaligned((d2 as f32).to_bits());
        rlo2
    }
});
