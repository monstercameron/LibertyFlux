// original: 0x00dd9270 UIBasicClip::vf140
/// `UIBasicClip::vf140`: forward one argument to the `field_1ec` member.
///
/// Loads the member pointer at `this+0x1ec`, reads its table pointer,
/// and tail-calls the entry at slot `0x20c`, forwarding the member and
/// the argument unchanged. Returns the target's answer.
export!(thiscall, rw_00dd9270(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
        let target = (((member as *const u32).read() + 0x20c) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(member, arg0)
    }
});
