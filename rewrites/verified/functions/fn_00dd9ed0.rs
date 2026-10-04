// original: 0x00dd9ed0 UIBasicClip::vf135
/// `UIBasicClip::vf135`: forward one argument to the `field_1ec` member.
///
/// Loads the member pointer at `this+0x1ec`, reads its table pointer,
/// and tail-calls the entry at slot `0x208`, forwarding the member and
/// the argument unchanged. Returns the target's answer.
export!(thiscall, rw_00dd9ed0(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
        let target = (((member as *const u32).read() + 0x208) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(member, arg0)
    }
});
