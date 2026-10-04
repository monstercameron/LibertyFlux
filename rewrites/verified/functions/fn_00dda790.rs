// original: 0x00dda790 UIBasicClip::vf126
/// Set or append the text of this clip's child element.
///
/// The child lives at +0x1E8. When the low byte of `mode` is zero, the
/// given string replaces the child's text through its slot-0x1E0 setter.
/// Otherwise the current text is read through the slot-0x20C getter: a null
/// answer falls back to replacing, and a live answer is copied into a
/// 256-byte scratch buffer; the new string is appended only when both plus
/// the terminator fit, then the buffer is handed to the setter. When the
/// combined text does not fit, the setter is skipped and the function
/// returns the remaining free space (256 minus the current length); on the
/// paths that call the setter its answer is returned.
export!(thiscall, rw_00dda790(this_ptr: u32, text: u32, mode: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x1E8;
        const SET_TEXT_SLOT: u32 = 0x1E0;
        const GET_TEXT_SLOT: u32 = 0x20C;
        const BUF_LEN: u32 = 256;

        let child = ((this_ptr.wrapping_add(CHILD_OFF)) as *const u32).read();
        let vtable = (child as *const u32).read();
        let set_text: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute((((vtable.wrapping_add(SET_TEXT_SLOT))) as *const u32).read() as usize);
        if (mode & 0xFF) == 0 {
            let answer = set_text(child, text, 0);
            let _: u32 = callee_stdcall!(4, u32,);
            return answer;
        }
        let get_text: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vtable.wrapping_add(GET_TEXT_SLOT))) as *const u32).read() as usize);
        if get_text(child) == 0 {
            let answer = set_text(child, text, 0);
            let _: u32 = callee_stdcall!(4, u32,);
            return answer;
        }
        let mut buf = [0u8; 256];
        let _: u32 = callee_cdecl!(3, u32, buf.as_mut_ptr() as u32, 0, BUF_LEN);
        let current = get_text(child);
        let mut n = 0usize;
        while ((current.wrapping_add(n as u32)) as *const u8).read() != 0 {
            buf[n] = ((current.wrapping_add(n as u32)) as *const u8).read();
            n += 1;
        }
        let cur_len = n as u32;
        let mut m = 0u32;
        while ((text.wrapping_add(m)) as *const u8).read() != 0 {
            m += 1;
        }
        let free = BUF_LEN.wrapping_sub(cur_len);
        if free <= m {
            let _: u32 = callee_stdcall!(4, u32,);
            return free;
        }
        let mut k = 0u32;
        while k <= m {
            buf[(cur_len.wrapping_add(k)) as usize] = ((text.wrapping_add(k)) as *const u8).read();
            k += 1;
        }
        let answer = set_text(child, buf.as_ptr() as u32, 0);
        let _: u32 = callee_stdcall!(4, u32,);
        answer
    }
});
