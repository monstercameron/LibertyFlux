// original: 0x00dda410 UIBasicClip::vf128
/// `UIBasicClip::vf128`: forward two arguments to the `field_1e8` member.
///
/// Loads the member pointer at `this+0x1e8`, reads its table pointer,
/// and tail-calls the entry at slot `0x1f8`, forwarding the member and
/// both arguments unchanged. Returns the target's answer.
export!(thiscall, rw_00dda410(this_ptr: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let target = (((member as *const u32).read() + 0x1f8) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(member, arg0, arg1)
    }
});
