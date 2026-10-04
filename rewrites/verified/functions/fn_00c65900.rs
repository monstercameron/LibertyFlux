// original: 0x00c65900 CCutsceneObject::vf21
/// Copies three words describing the object into the caller's buffer.
///
/// Source is the attached record (offset 0x20, plus 0x30) when one is
/// attached, otherwise the inline words at offset 0x10.
export!(thiscall, rw_c65900(this: u32, out: u32) -> u32 {
    let attached = unsafe { (this as *const u32).byte_add(0x20).read() };
    let src = if attached != 0 {
        attached.wrapping_add(0x30)
    } else {
        this.wrapping_add(0x10)
    };
    unsafe {
        let src = src as *const u32;
        let dst = out as *mut u32;
        dst.write(src.read());
        dst.add(1).write(src.add(1).read());
        dst.add(2).write(src.add(2).read());
    }
    out
});
