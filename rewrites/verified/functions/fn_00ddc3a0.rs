// original: 0x00ddc3a0 uitext_fit_string_ellipsis (proposed)
//
// (Also used as the template for 0x00ddc4f0, which differs only in the member
// offset it loads the string object from. See STR_MEMBER.)

/// Fit a string object's text into this container's width by truncating with
/// an ellipsis.
///
/// `this` points to the container object. Its member at `+STR_MEMBER` points
/// to a string object with a virtual table; the slots used are `+VT_GET_TEXT`
/// (returns the current text as a NUL-terminated byte string),
/// `+VT_SET_TEXT` (takes a new text pointer and a flags word),
/// `+VT_MEASURE` (recomputes layout after the text changes) and `+VT_WIDTH`
/// (returns the laid-out width as an x87 float). The container's own table
/// slot `+VT_WIDTH` returns the available width, also on the x87 stack.
///
/// Algorithm: copy the current text into a freshly allocated buffer, stamp an
/// ellipsis over its tail when it is longer than three bytes, install the
/// buffer, measure, and while the laid-out width exceeds the available width,
/// chop one byte at a time (keeping the ellipsis suffix) and re-install and
/// re-measure. The scratch buffer is freed before returning.
///
/// Edge cases, all matching the original exactly: an empty or short text
/// (length 3 or less) gets no initial ellipsis; each loop pass ends the
/// string one byte earlier and re-adds dots to taste (`...` while the
/// remainder is longer than three bytes, fewer dots below that, re-terminated
/// each time); a length that drops below zero ends the loop; a NaN on either
/// side of the width comparison ends the loop too (unordered compares exit);
/// the initial ellipsis stamp of a 4+ byte string overwrites the terminator,
/// so the first installed buffer is unterminated, byte for byte as observed.
///
/// Original: thiscall, no stack arguments, no return value.
lf_checker_rt::export!(thiscall, rw_00ddc3a0(this: u32) -> u32 {
    unsafe {
        const STR_MEMBER: u32 = 0x1f8;
        const VT_GET_TEXT: u32 = 0x20c;
        const VT_SET_TEXT: u32 = 0x1e0;
        const VT_MEASURE: u32 = 0x14c;
        const VT_WIDTH: u32 = 0x8c;
        const MALLOC_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const DOT: u8 = 0x2e;
        const SHORT_TEXT: i32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let str_obj = rd32(this.wrapping_add(STR_MEMBER));
        let str_vtable = rd32(str_obj);
        let get_text: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(VT_GET_TEXT)) as usize) };
        let set_text: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(VT_SET_TEXT)) as usize) };
        let measure: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(VT_MEASURE)) as usize) };
        let str_width: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(VT_WIDTH)) as usize) };
        let this_vtable = rd32(this);
        let this_width: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(this_vtable.wrapping_add(VT_WIDTH)) as usize) };

        let first = get_text(str_obj);
        let mut len: u32 = 0;
        while rd8(first.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }
        let buf: u32 = lf_checker_rt::callee_cdecl!(MALLOC_CALLEE, u32, len.wrapping_add(1));
        let second = get_text(str_obj);
        let mut i: u32 = 0;
        loop {
            let c = rd8(second.wrapping_add(i));
            wr8(buf.wrapping_add(i), c);
            i = i.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        if (len as i32) > SHORT_TEXT {
            wr8(buf.wrapping_add(len).wrapping_sub(2), DOT);
            wr8(buf.wrapping_add(len).wrapping_sub(1), DOT);
            wr8(buf.wrapping_add(len), DOT);
        }
        set_text(str_obj, buf, 0);
        measure(str_obj);
        let mut inner = str_width(str_obj);
        let mut outer = this_width(this);
        let mut cur = len;
        while inner > outer {
            if (cur as i32) < 0 {
                break;
            }
            wr8(buf.wrapping_add(cur), 0);
            cur = cur.wrapping_sub(1);
            if (cur as i32) > SHORT_TEXT {
                wr8(buf.wrapping_add(cur).wrapping_sub(2), DOT);
            } else {
                if (cur as i32) >= 0 {
                    wr8(buf.wrapping_add(1), DOT);
                }
                if (cur as i32) >= 1 {
                    wr8(buf.wrapping_add(2), DOT);
                }
                wr8(buf.wrapping_add(cur).wrapping_add(1), 0);
            }
            set_text(str_obj, buf, 0);
            measure(str_obj);
            inner = str_width(str_obj);
            outer = this_width(this);
        }
        lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, buf);
        0
    }
});
