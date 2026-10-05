// original: 0x00c05b10 stream_request_dispatch
/// Dispatch a streaming request by its mode field.
///
/// Returns quietly when the flag at `this`+1 is set, when the request id at
/// +8 is -1, or when the lookup helper (cdecl/2: id, 1) answers null. For a
/// mode at +0xc of -3, -2 or -1, publishes 0x27, 0x25 or 0x14 to the mode
/// global, notifies through the notify helper (thiscall/1 on the fixed
/// notify object with the lookup answer) and restores the global. For any
/// other mode, queries through the query helper (thiscall/1 on the fixed
/// query object) and hands the answer's address to the consume helper
/// (cdecl/1, taking a pointer to this function's own stack slot, whose
/// address is skipped and whose content is snap-compared). Returns nothing.
/// Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c05b10(this: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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

        const LOOKUP: u32 = 1;
        const QUERY: u32 = 2;
        const CONSUME: u32 = 3;
        const NOTIFY: u32 = 4;
        const QUERY_OBJ: u32 = 0x0115D9A0;
        const NOTIFY_OBJ: u32 = 0x012831E4;
        const MODE_GLOBAL: u32 = 0x012831EC;
        const NONE: i32 = -1;
        if rd8(this + 1) != 0 {
            return 0;
        }
        let id = rd32(this + 8) as i32;
        if id == NONE {
            return 0;
        }
        let handle: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id as u32, 1);
        if handle == 0 {
            return 0;
        }
        let mode = rd32(this + 0x0c) as i32;
        if mode == -3 || mode == -2 || mode == -1 {
            let saved = rd32(lf_checker_rt::relocated(MODE_GLOBAL));
            let published: u32 = if mode == -3 { 0x27 } else if mode == -2 { 0x25 } else { 0x14 };
            wr32(lf_checker_rt::relocated(MODE_GLOBAL), published);
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, lf_checker_rt::relocated(NOTIFY_OBJ), handle);
            wr32(lf_checker_rt::relocated(MODE_GLOBAL), saved);
            return 0;
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32, lf_checker_rt::relocated(QUERY_OBJ), mode as u32);
        let slot = answer;
        let _: u32 = lf_checker_rt::callee_cdecl!(CONSUME, u32, &slot as *const u32 as u32);
        0
    }
});
