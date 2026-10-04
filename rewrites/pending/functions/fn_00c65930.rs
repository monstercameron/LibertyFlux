// original: 0x00c65930 CCutsceneObject::vf20
/// Copies the 16-byte pose record into the caller's buffer.
///
/// Resolves the record through vtable slot 0x54 (handed a scratch word),
/// then copies it: integer words at 0 and 0xc, floats at 4 and 8. Returns
/// the trailing integer word.
export!(thiscall, rw_c65930(this: u32, out: u32) -> u32 {
    let vtable = unsafe { (this as *const u32).read() };
    let slot = unsafe { (vtable as *const u32).byte_add(0x54).read() };
    let target: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    let mut scratch = [0u32; 4];
    let got = target(this, scratch.as_mut_ptr() as u32);
    unsafe {
        let src = got as *const u32;
        let dst = out as *mut u32;
        dst.write(src.read());
        (dst.add(1) as *mut f32).write((src.add(1) as *const f32).read());
        (dst.add(2) as *mut f32).write((src.add(2) as *const f32).read());
        let tail = src.add(3).read();
        dst.add(3).write(tail);
        tail
    }
});
