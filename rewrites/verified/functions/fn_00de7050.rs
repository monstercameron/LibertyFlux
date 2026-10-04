// original: 0x00de7050 UITimeOverview::build_rows (proposed)

/// Build the eight timed rows plus the end row of the time-overview panel.
///
/// `this` is the panel object (thiscall, no stack arguments). For each of
/// eight rows the function allocates a small descriptor (0x25c bytes) and a
/// row object (0x610 bytes) through the allocator callee, asks the panel's
/// builder hook (vtable slot +0x48) for a token, turns the token into a
/// component through the make callee (which takes only the format string),
/// constructs the objects, stores them at `this+0x1e0+4*row` and
/// `this+0x200+4*row`, then lays the row out: a layout call with a scratch
/// word, a step call carrying the row height (18.0f as bits) and the layout
/// result, a colour call with an opaque-black word, a zero-time label call
/// ("00:00:00"), and three flag calls (1, 0, 0x12). After the loop an end
/// row is built the same way (without the make step) and stored at
/// `this+0x220`; `this+0x224` is set to 1 and `this+0x22c` to 0, and the
/// panel's finish hook (slot +0x13c) runs with argument 1. Its result is
/// the function's return value.
///
/// A null small descriptor stores 0 and continues; a null row object faults
/// on the unconditional vtable read, which the rewrite reproduces with a
/// raw read so fault trials match. The format strings live in the image and
/// are derived with `relocated`, never hard-coded. No floating-point
/// arithmetic is performed (float-shaped words are only passed or stored).
///
/// Original: 0x00de7050 (thiscall, no stack words, plain `ret`).
lf_checker_rt::export!(thiscall, rw_00de7050(this: u32) -> u32 {
    unsafe {
        const ROW_COUNT: u32 = 8;
        const FIELD_FIRST: u32 = 0x1e0;
        const FIELD_ROW: u32 = 0x200;
        const FIELD_END: u32 = 0x220;
        const FIELD_STATE: u32 = 0x224;
        const FIELD_FLAG: u32 = 0x22c;
        const VT_BUILD: u32 = 0x48;
        const VT_FINISH: u32 = 0x13c;
        const VT_STEP: u32 = 0x1cc;
        const VT_COLOUR: u32 = 0x208;
        const VT_LABEL: u32 = 0x1e0;
        const VT_F1: u32 = 0x1fc;
        const VT_F0: u32 = 0x1f8;
        const VT_F12: u32 = 0x1d4;
        const SMALL_SIZE: u32 = 0x25c;
        const ROW_SIZE: u32 = 0x610;
        const TEX_FMT: u32 = 0xefe7fc;
        const TXT_FMT: u32 = 0xefe810;
        const ZERO_TIME: u32 = 0xefe824;
        const END_FMT: u32 = 0xefe830;
        const OPAQUE_BLACK: u32 = 0xff000000;
        const ROW_HEIGHT_BITS: u32 = 0x41900000;
        const LAYOUT_TAG: u32 = 7;
        const FINISH_ARG: u32 = 1;
        const NEW1: u32 = 1;
        const NEW2: u32 = 2;
        const NEW3: u32 = 3;
        const MAKE: u32 = 5;
        const CTOR_SMALL: u32 = 6;
        const CTOR_END: u32 = 7;
        const PLACE: u32 = 8;
        const CTOR_ROW: u32 = 9;
        const LAYOUT: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn build_hook(this: u32) -> u32 {
            unsafe {
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(VT_BUILD)) as usize);
                hook(this)
            }
        }

        let mut row = 0u32;
        let mut field = this.wrapping_add(FIELD_ROW);
        while row < ROW_COUNT {
            let mem_small: u32 = lf_checker_rt::callee_cdecl!(NEW1, u32, SMALL_SIZE);
            let first: u32;
            if mem_small != 0 {
                let token = build_hook(this);
                let _ = token;
                let made: u32 =
                    lf_checker_rt::callee_stdcall!(MAKE, u32, lf_checker_rt::relocated(TEX_FMT));
                first = lf_checker_rt::callee_thiscall!(CTOR_SMALL, u32, mem_small, made);
            } else {
                first = 0;
            }
            wr32(field.wrapping_sub(FIELD_ROW - FIELD_FIRST), first);
            let mut style: u32 = OPAQUE_BLACK;
            let style_ptr = (&mut style as *mut u32) as u32;
            let _: u32 =
                lf_checker_rt::callee_thiscall!(PLACE, u32, first, 0u32, 0u32, style_ptr, 0xffff_ffffu32);

            let mem_row: u32 = lf_checker_rt::callee_cdecl!(NEW2, u32, ROW_SIZE);
            let row_obj: u32;
            if mem_row != 0 {
                let token = build_hook(this);
                let _ = token;
                let made: u32 =
                    lf_checker_rt::callee_stdcall!(MAKE, u32, lf_checker_rt::relocated(TXT_FMT));
                row_obj = lf_checker_rt::callee_thiscall!(CTOR_ROW, u32, mem_row, made);
            } else {
                row_obj = 0;
            }
            wr32(field, row_obj);
            // Unconditional vtable read: faults when the row object is null,
            // exactly like the original, so fault trials match.
            let row_vt = rd32(row_obj);
            let mut scratch: u32 = 0;
            let scratch_ptr = (&mut scratch as *mut u32) as u32;
            let laid: u32 =
                lf_checker_rt::callee_thiscall!(LAYOUT, u32, 0u32, scratch_ptr, LAYOUT_TAG);
            let step: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_STEP)) as usize);
            let _ = step(row_obj, ROW_HEIGHT_BITS, laid);
            let colour: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_COLOUR)) as usize);
            let mut ink: u32 = OPAQUE_BLACK;
            let ink_ptr = (&mut ink as *mut u32) as u32;
            let _ = colour(row_obj, ink_ptr);
            let label: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_LABEL)) as usize);
            let _ = label(row_obj, lf_checker_rt::relocated(ZERO_TIME), 0u32);
            let flag1: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_F1)) as usize);
            let _ = flag1(row_obj, 1u32);
            let flag0: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_F0)) as usize);
            let _ = flag0(row_obj, 0u32);
            let flag12: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(row_vt.wrapping_add(VT_F12)) as usize);
            let _ = flag12(row_obj, 0x12u32);

            row += 1;
            field = field.wrapping_add(4);
        }

        let mem_end: u32 = lf_checker_rt::callee_cdecl!(NEW3, u32, SMALL_SIZE);
        let end_obj: u32;
        if mem_end != 0 {
            let token = build_hook(this);
            end_obj =
                lf_checker_rt::callee_thiscall!(CTOR_END, u32, mem_end, lf_checker_rt::relocated(END_FMT), token);
        } else {
            end_obj = 0;
        }
        wr32(this.wrapping_add(FIELD_END), end_obj);
        let mut end_style: u32 = OPAQUE_BLACK;
        let end_style_ptr = (&mut end_style as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(PLACE, u32, end_obj, 0u32, 0u32, end_style_ptr, 0xffff_ffffu32);
        wr32(this.wrapping_add(FIELD_FLAG), 0);
        wr32(this.wrapping_add(FIELD_STATE), FINISH_ARG);
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(VT_FINISH)) as usize);
        finish(this, FINISH_ARG)
    }
});
