// original: 0x00dd9670 UIMontageClip::vf108
/// `UIMontageClip::vf108`: clear path with an optional extra member.
///
/// Runs the `field_1ec` member's slot `0x120` with 0, additionally the
/// `field_1e4` member's when `flag_2F8` is 1, resolves helper value `0x3b`
/// for the `field_1e8` member's slot `0x208`, and finishes through the own
/// slot `0x20c` with 0. Returns the final answer.
export!(thiscall, rw_00dd9670(this_ptr: u32) -> u32 {
    unsafe {
        let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
        let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
        let first: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        first(member, 0);
        if ((this_ptr as *const u8).add(0x2f8)).read() == 1 {
            let member = ((this_ptr as *const u32).add(0x1e4 / 4)).read();
            let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
            let extra: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            extra(member, 0);
        }
        let scratch = 0u32;
        let r = callee_cdecl!(2, u32, &scratch as *const u32 as u32, 0x3b);
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
        let set: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        set(member, r);
        let slot = ((((this_ptr as *const u32).read() + 0x20c)) as *const u32).read();
        let fin: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        fin(this_ptr, 0)
    }
});
