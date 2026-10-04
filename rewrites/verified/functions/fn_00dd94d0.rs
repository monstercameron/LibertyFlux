// original: 0x00dd94d0 UIMontageClip::vf107
/// `UIMontageClip::vf107`: apply the mode selected by `flag_2F8`.
///
/// Mode 2 pre-selects through the `field_1ec` member, mode 1 post-selects
/// through `field_1e4`, any other mode does neither. All modes resolve one
/// helper value (constant 2 in mode 2, `0x3e` otherwise), pass it to the
/// `field_1e8` member's slot `0x208`, and finish through the object's own
/// slot `0x20c`. Returns the final answer.
export!(thiscall, rw_00dd94d0(this_ptr: u32) -> u32 {
    unsafe {
        let flag = ((this_ptr as *const u8).add(0x2f8)).read();
        let k: u32 = if flag == 2 {
            let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
            let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
            let pre: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            pre(member, 1);
            2
        } else {
            0x3e
        };
        let scratch = 0u32;
        let r = callee_cdecl!(1, u32, &scratch as *const u32 as u32, k);
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
        let set: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        set(member, r);
        if flag == 1 {
            let member = ((this_ptr as *const u32).add(0x1e4 / 4)).read();
            let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
            let post: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            post(member, 1);
        }
        let slot = ((((this_ptr as *const u32).read() + 0x20c)) as *const u32).read();
        let fin: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        fin(this_ptr, 1)
    }
});
