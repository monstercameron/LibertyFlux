// original: 0x00dd34b0 UIMontageContainer::vf100
/// Per-item montage visibility update.
///
/// Resolves the montage source object for `arg` through the UI registry,
/// derives two axis-aligned rectangles from float getters at vtable slots
/// 0xB8/0xC0/0xC8/0xD0 (each scaled by the constant 0.5), keeps the
/// component-wise maxima, and checks the merged box against the clamped
/// viewport (screen rows/cols scaled and clamped to [0, 1]). If the box is
/// inside, verifies the source is a `UIMontageClip` by comparing its tag
/// against the registered id and, on match, forwards `arg` to the child at
/// +0x1E0 slot 0x1E4 and stores the result at +0x1FC.
///
/// Returns nothing meaningful (the original leaves EAX untouched on its
/// early-out paths); the contract compares calls and written memory.
///
/// Status (lane r-b164): believed correct but NOT verified. The stock
/// checker runs the original and this rewrite from identical requests yet
/// reaches different gate outcomes on some trials (see the lane report:
/// orig-side float answers diverge from the scripted values). Kept here
/// for re-run under a fixed checker.
export!(thiscall, rw_00dd34b0(this_obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn vcall_f32(obj: u32, slot: usize) -> f32 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            f(obj)
        }
        #[inline(always)]
        unsafe fn vcall_u32(obj: u32, slot: usize) -> u32 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            f(obj)
        }
        #[inline(always)]
        unsafe fn vcall_u32_1(obj: u32, slot: usize, arg: u32) -> u32 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(slot) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(obj, arg)
        }
        if *this_obj.add(0x21C) != 0 {
            return 0;
        }
        let half = f32::from_bits(*global::<u32>(0x00FE8830));
        let one = f32::from_bits(*global::<u32>(0x00FE88E8));
        let src = callee_thiscall!(1, u32, relocated(0x01981A4C), arg);
        // Source-object terms.
        let t_c = vcall_f32(src, 0xC8);
        let t0 = vcall_f32(src, 0xB8);
        let s10 = t_c - t0 * half;
        let t1 = vcall_f32(src, 0xB8);
        let d = t1 * half;
        let t2 = vcall_f32(src, 0xC8);
        let s18 = t2 + d;
        let t3 = vcall_f32(src, 0xD0);
        let t4 = vcall_f32(src, 0xC0);
        let s20 = t3 - t4 * half;
        let t5 = vcall_f32(src, 0xC0);
        let e = t5 * half;
        let t6 = vcall_f32(src, 0xD0);
        let s28 = t6 + e;
        // Container's own terms.
        let this_u = this_obj as u32;
        let k = vcall_f32(this_u, 0xC8);
        let l = vcall_f32(this_u, 0xB8);
        let s14 = k - l * half;
        let m = vcall_f32(this_u, 0xB8);
        let n = m * half;
        let o = vcall_f32(this_u, 0xC8);
        let s1c = o + n;
        let p = vcall_f32(this_u, 0xD0);
        let q = vcall_f32(this_u, 0xC0);
        let s24 = p - q * half;
        let r = vcall_f32(this_u, 0xC0);
        let s = r * half;
        let t = vcall_f32(this_u, 0xD0);
        // Component-wise maxima (each `jbe` keeps the first operand unless
        // the second is strictly greater; NaN keeps the first side's value).
        let x7 = if s14 > s10 { s14 } else { s10 };
        let x1 = t + s;
        let x6 = if s18 > s1c { s18 } else { s1c };
        let x5 = if s20 > s24 { s20 } else { s24 };
        let x4 = if x1 > s28 { x1 } else { s28 };
        // Viewport extents, scaled and clamped to [0, 1] (NaN survives).
        let h_raw = (*global::<i32>(0x018B7A8C) as f32)
            * f32::from_bits(*global::<u32>(0x017ACCF0));
        let h = if 0.0 > h_raw {
            0.0
        } else if h_raw > one {
            one
        } else {
            h_raw
        };
        let w_raw = (*global::<i32>(0x018B7A80) as f32)
            * f32::from_bits(*global::<u32>(0x017ACCE8));
        let w = if 0.0 > w_raw {
            0.0
        } else if w_raw > one {
            one
        } else {
            w_raw
        };
        // Inside test: strict on all four sides (NaN fails).
        if !(w > x7) {
            return 0;
        }
        if !(x6 > w) {
            return 0;
        }
        if !(h > x5) {
            return 0;
        }
        if !(x4 > h) {
            return 0;
        }
        let tag = vcall_u32(src, 0);
        let want: u32 = callee_cdecl!(11, u32, relocated(0x00EFAEAC));
        if tag != want {
            return 0;
        }
        let _: u32 = callee_thiscall!(12, u32, relocated(0x01176888), relocated(0x00EFAEBC));
        let child = *(this_obj.add(0x1E0) as *const u32);
        let out = vcall_u32_1(child, 0x1E4, arg);
        *(this_obj.add(0x1FC) as *mut u32) = out;
        0
    }
});
