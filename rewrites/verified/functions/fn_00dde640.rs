// original: 0x00DDE640 UITextField cursor clamp
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Clamp the cursor at `+0x208`. Add the low byte of `delta` to the cursor
/// (wrapping); when the sum exceeds the text length (both compared as SIGNED
/// bytes), store the length instead, otherwise store the sum, or 0 when the
/// sum is negative. The text length is measured by scanning for the NUL.
/// Returns nothing. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00DDE640(this: u32, delta: u32) -> u32 {
    unsafe {
        const CURSOR: u32 = 0x208;
        const TEXT_GETTER: u32 = 1;
        unsafe fn text_len(this: u32) -> u32 {
            let s = lf_checker_rt::callee_thiscall!(TEXT_GETTER, u32, this);
            let mut p = s;
            while ((p) as *const u8).read() != 0 {
                p = p.wrapping_add(1);
            }
            p.wrapping_sub(s)
        }
        let base = ((this + CURSOR) as *const u8).read();
        let sum = base.wrapping_add(delta as u8);
        let len = text_len(this);
        if (sum as i8) <= ((len as u8) as i8) {
            let v = if (sum as i8) < 0 { 0u8 } else { sum };
            ((this + CURSOR) as *mut u8).write(v);
        } else {
            let len2 = text_len(this);
            ((this + CURSOR) as *mut u8).write(len2 as u8);
        }
        0
    }
});
