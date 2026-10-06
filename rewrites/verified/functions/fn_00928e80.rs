// original: 0x00928E80 CRenderPhaseCascadeShadows::vf3 (symbols)

/// Virtual slot 3 of `CRenderPhaseCascadeShadows`: run the cascade pass when
/// the frame's distance test fails.
///
/// Returns at once when the enable byte at `ENABLE` is clear (the original
/// returns its entry `eax` there, which a rewrite cannot observe, so the
/// contract compares no return value). Otherwise selects the frame index
/// like `current_frame_slot_ptr` (TLS flags at `+0x8d0` choose between
/// `FRAME_A` and `FRAME_B`), loads the frame's distance float from
/// `DIST_BASE + index * DIST_STRIDE`, and returns at once when it is equal
/// to the constant at `DIST_REF` (an unordered/NaN comparison runs the pass,
/// matching `ucomiss`). The pass calls the setup helper (callee 1) with
/// `(this+0x900, this+0x8f8, this+0x8fc, this, 1, 1)`, the allocator
/// (callee 2, thiscall on `ALLOC_OBJ`), stores its answer at `+0x938` and
/// passes it to the submit helper (callee 3) with `(answer, 0)`, then
/// tail-calls the finish routine (callee 4).
///
/// Original: 0x00928E80 (thiscall, no stack arguments). Three direct calls
/// plus an E9 tail jump.
lf_checker_rt::export!(thiscall, rw_00928E80(this: u32) -> u32 {
    unsafe {
        const ENABLE: u32 = 0x0103_6AD0;
        const TLS_INDEX: u32 = 0x017A_BA14;
        const FLAGS_OFF: u32 = 0x8d0;
        const FRAME_A: u32 = 0x0117_4790;
        const FRAME_B: u32 = 0x0117_4794;
        const DIST_BASE: u32 = 0x0154_CC00;
        const DIST_STRIDE: u32 = 128;
        const DIST_REF: u32 = 0x00FE_8628;
        const ALLOC_OBJ: u32 = 0x0161_4C90;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        let flag = (lf_checker_rt::relocated(ENABLE) as *const u8).read();
        if flag == 0 {
            return 0;
        }
        let idx = rd(lf_checker_rt::relocated(TLS_INDEX));
        let tls = lf_checker_rt::tls_slot(idx as usize);
        let flags = rd(tls.wrapping_add(FLAGS_OFF));
        let sel = if (flags >> 1) & 1 != 0 {
            rd(lf_checker_rt::relocated(FRAME_A))
        } else if (flags >> 3) & 1 != 0 {
            rd(lf_checker_rt::relocated(FRAME_A))
        } else {
            rd(lf_checker_rt::relocated(FRAME_B))
        };
        let dist = f32::from_bits(rd(
            sel.wrapping_mul(DIST_STRIDE)
                .wrapping_add(lf_checker_rt::relocated(DIST_BASE)),
        ));
        let limit = f32::from_bits(rd(lf_checker_rt::relocated(DIST_REF)));
        if core::hint::black_box(dist) == core::hint::black_box(limit) {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            this.wrapping_add(0x900),
            this.wrapping_add(0x8f8),
            this.wrapping_add(0x8fc),
            this,
            1u32,
            1u32
        );
        let handle: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(ALLOC_OBJ));
        (this.wrapping_add(0x938) as *mut u32).write_unaligned(handle);
        lf_checker_rt::callee_cdecl!(3, u32, handle, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32,)
    }
});
