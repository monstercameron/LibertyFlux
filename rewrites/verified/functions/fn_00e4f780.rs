// original: 0x00E4F780 UIGalleryFileViewer::vf97
/// UIGalleryFileViewer::vf97: refresh the gallery viewer against the file list.
///
/// Looks up the viewer object for the incoming argument, samples four pairs of
/// gauge readings from it and from the sibling object at `this + 0x32C`,
/// combines each pair with a global scale factor, clamps two global counters
/// into a display range, and — when every combined gauge falls inside the
/// range — rebuilds the viewer chain and records the viewer at `this + 0x350`.
/// Otherwise reports through `this + 0x1A4` and leaves the chain untouched.
///
/// Two transports differ from the original (same values, documented in the
/// contract): the hidden second stack word the original reads above its
/// declared argument is mirrored into a heap word, and the caller-clean
/// slot-0x54 call is made fastcall-style (object in ECX, nothing pushed)
/// while the original pushes its incoming EBP there (that argument is
/// skipped in the call comparison; EBP is not observable from Rust).
lf_checker_rt::export!(thiscall, rw_e4f780(this: u32, arg: u32) -> u32 {
    unsafe {
        // Table-slot calls: load the target from [base + slot], object in ECX.
        let vcall0 = |base: u32, slot: usize, this: u32| unsafe {
            let target = *((base as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this)
        };
        let vcall1 = |base: u32, slot: usize, this: u32, arg: u32| unsafe {
            let target = *((base as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this, arg)
        };
        let vcall0f = |base: u32, slot: usize, this: u32| unsafe {
            let target = *((base as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            f(this)
        };
        let load_vt = |obj: u32| unsafe { *(obj as *const u32) };
        let hub = lf_checker_rt::relocated(0x01981A4C);
        // Entry: announce, then resolve the viewer object for the argument.
        vcall1(load_vt(this), 0x18, this, 1);
        let viewer = lf_checker_rt::callee_thiscall!(1, u32, hub, arg);
        if viewer == 0 {
            return 0;
        }
        let sibling = *((this as *const u8).add(0x32C) as *const u32);
        if sibling == 0 {
            return viewer;
        }
        // Gauge phase: eight float samples per object; each consecutive pair
        // (direct, scaled) is combined with the global scale factor.
        let gain = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE8830));
        let vvt = load_vt(viewer);
        let a0 = vcall0f(vvt, 0xC8, viewer);
        let b0 = vcall0f(vvt, 0xB8, viewer);
        let g0 = a0 - b0 * gain;
        let t = vcall0f(vvt, 0xB8, viewer) * gain;
        let g1 = vcall0f(vvt, 0xC8, viewer) + t;
        let c0 = vcall0f(vvt, 0xD0, viewer);
        let d0 = vcall0f(vvt, 0xC0, viewer);
        let g2 = c0 - d0 * gain;
        let t = vcall0f(vvt, 0xC0, viewer) * gain;
        let g3 = vcall0f(vvt, 0xD0, viewer) + t;
        let svt = load_vt(sibling);
        let a1 = vcall0f(svt, 0xC8, sibling);
        let b1 = vcall0f(svt, 0xB8, sibling);
        let h0 = a1 - b1 * gain;
        let t = vcall0f(svt, 0xB8, sibling) * gain;
        let h1 = vcall0f(svt, 0xC8, sibling) + t;
        let c1 = vcall0f(svt, 0xD0, sibling);
        let d1 = vcall0f(svt, 0xC0, sibling);
        let h2 = c1 - d1 * gain;
        let t = vcall0f(svt, 0xC0, sibling) * gain;
        let h3 = vcall0f(svt, 0xD0, sibling) + t;
        // Cross-object extrema. Each mirrors one comiss + conditional move:
        // the move runs only on ordered-above, so `>` (false for NaN, like
        // the not-taken jump) reproduces the original exactly.
        let mut lo0 = g0;
        if h0 > lo0 {
            lo0 = h0;
        }
        let mut hi0 = g1;
        if hi0 > h1 {
            hi0 = h1;
        }
        let mut lo1 = g2;
        if h2 > lo1 {
            lo1 = h2;
        }
        let mut hi1 = g3;
        if hi1 > h3 {
            hi1 = h3;
        }
        // Range from the two global counters, each clamped to [0, limit].
        let limit = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE88E8));
        let mut r0 = (*lf_checker_rt::global::<i32>(0x018B7A8C) as f32)
            * f32::from_bits(*lf_checker_rt::global::<u32>(0x017ACCF0));
        if 0.0 > r0 {
            r0 = 0.0;
        } else if r0 > limit {
            r0 = limit;
        }
        let raw1 = (*lf_checker_rt::global::<i32>(0x018B7A80) as f32)
            * f32::from_bits(*lf_checker_rt::global::<u32>(0x017ACCE8));
        let mut r1 = 0.0f32;
        if !(0.0 > raw1) {
            if raw1 > limit {
                r1 = limit;
            } else {
                r1 = raw1;
            }
        }
        // Gate: every gauge must fall strictly inside the range.
        let tvt = load_vt(this);
        if !(r1 > lo0) {
            return vcall0(tvt, 0x1A4, this);
        }
        if !(hi0 > r1) {
            return vcall0(tvt, 0x1A4, this);
        }
        if !(r0 > lo1) {
            return vcall0(tvt, 0x1A4, this);
        }
        if !(hi1 > r0) {
            return vcall0(tvt, 0x1A4, this);
        }
        // Rebuild phase: resolve the chain links, then relink and report.
        // Slot 0x54 is caller-clean (the pushed word stays live until the
        // function epilogue), so it is invoked fastcall-style: ECX carries
        // the object, no stack argument is pushed, and neither side cleans.
        let step = {
            let target = *((vvt as *const u8).add(0x54) as *const u32);
            let f: extern "fastcall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(viewer, 0)
        };
        let chain = lf_checker_rt::callee_thiscall!(2, u32, hub, step);
        let cvt = load_vt(chain);
        let pick = vcall0(cvt, 0x54, chain);
        let third = lf_checker_rt::callee_thiscall!(3, u32, hub, pick);
        if vcall0(vvt, 0x1C, viewer) & 0xFF == 0 {
            lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(0x01176888), lf_checker_rt::relocated(0x00F19B4C));
        }
        let gate = *((this as *const u8).add(0x328) as *const u32);
        if gate != 0 {
            lf_checker_rt::callee_thiscall!(5, u32, gate, viewer);
        }
        let avt = load_vt(third);
        let node = vcall0(avt, 0x224, third);
        let nvt = load_vt(node);
        let leaf = vcall0(nvt, 0x220, node);
        vcall0(load_vt(leaf), 0x1B0, leaf);
        let leaf2 = vcall0(nvt, 0x220, node);
        vcall1(load_vt(leaf2), 0x18, leaf2, 0);
        vcall1(nvt, 0x18, node, 0);
        let mark = vcall1(cvt, 0x4C, chain, 1);
        // Hidden word: the original reads one stack word above its declared
        // argument; the contract mirrors that word into this heap slot.
        let hidden = *((this as *const u8).add(0x340) as *const u32);
        let table = load_vt(third);
        let tag = vcall1(table, 0x1E4, hidden, mark);
        vcall1(table, 0x230, hidden, tag);
        vcall1(cvt, 0x18, chain, 1);
        let flag = vcall1(vvt, 0x4C, viewer, 1);
        let ctable = load_vt(chain);
        let back = vcall1(ctable, 0x1E4, chain, flag);
        vcall1(ctable, 0x22C, chain, back);
        vcall0(vvt, 0x1AC, viewer);
        let code = vcall1(vvt, 0x18, viewer, 1);
        *((this as *mut u8).add(0x350) as *mut u32) = viewer;
        code
    }
});
