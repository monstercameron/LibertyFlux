// original: 0x00dd92e0 UIBasicClip::vf145
/// `UIBasicClip::vf145`: forward one argument to the `field_1e8` member.
///
/// Loads the member pointer at `this+0x1e8`, reads its table pointer,
/// and tail-calls the entry at slot `0x11c`, forwarding the member and
/// the argument unchanged. Returns the target's answer.
export!(thiscall, rw_00dd92e0(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let target = (((member as *const u32).read() + 0x11c) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(member, arg0)
    }
});
