// original: 0x00c65970 CCutsceneObject::vf23
/// Copies the 16-byte pose record, then runs the slot-0x58 follow-up.
///
/// Same record copy as vf20, then a no-argument call through vtable slot
/// 0x58 whose result is returned.
export!(thiscall, rw_c65970(this: u32, out: u32) -> u32 {
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
        dst.add(3).write(src.add(3).read());
        let follow = (vtable as *const u32).byte_add(0x58).read();
        let follow_up: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(follow as usize);
        follow_up(this)
    }
});
