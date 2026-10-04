// original: 0x00dd92a0 UIBasicClip::vf147
/// `UIBasicClip::vf147`: copy one word through two levels of members.
///
/// Stores `this->field_1e0->field_1e0` into the caller's output word and
/// returns the output pointer it was given.
export!(thiscall, rw_00dd92a0(this_ptr: u32, out_ptr: u32) -> u32 {
    unsafe {
        let inner = ((this_ptr as *const u32).add(0x1e0 / 4)).read();
        let value = ((inner as *const u32).add(0x1e0 / 4)).read();
        (out_ptr as *mut u32).write(value);
        out_ptr
    }
});
