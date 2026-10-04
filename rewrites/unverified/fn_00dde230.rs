// original: 0x00DDE230 is_text_empty
/// Report whether the field's current text is the empty string (1) or not
/// (0), by measuring the text the field resolves through its getter.
lf_checker_rt::export!(thiscall, rw_dde230(this: u32) -> u32 {
    unsafe {
        let text = lf_checker_rt::callee_thiscall!(1, u32, this);
        let mut p = text as *const u8;
        while *p != 0 {
            p = p.add(1);
        }
        if (p as u32).wrapping_sub(text) == 0 {
            1
        } else {
            0
        }
    }
});
