// original: 0x00dd96e0 UIClip::vf108
/// `UIClip::vf108`: clear path with an optional member and a gated block.
///
/// Runs the `field_1e4` member's slot `0x120` with 0 only when `flag_321` is
/// 1, resolves helper value `0x3b` for the `field_1e8` member's slot `0x208`,
/// and when the `field_1f0` member's slot `0x124` check passes resolves a
/// second `0x3b` value for the `field_1ec` member's slot `0x208`. Finishes
/// through the shared helper with 0 and returns its answer.
export!(thiscall, rw_00dd96e0(this_ptr: u32) -> u32 {
    unsafe {
        if ((this_ptr as *const u8).add(0x321)).read() == 1 {
            let member = ((this_ptr as *const u32).add(0x1e4 / 4)).read();
            let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
            let opt: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            opt(member, 0);
        }
        let scratch = 0u32;
        let fp = &scratch as *const u32 as u32;
        let r = callee_cdecl!(1, u32, fp, 0x3b);
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
        let set: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        set(member, r);
        let member = ((this_ptr as *const u32).add(0x1f0 / 4)).read();
        let slot = (((member as *const u32).read() + 0x124) as *const u32).read();
        let gate: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(slot as usize);
        if gate(member) != 0 {
            let r2 = callee_cdecl!(1, u32, fp, 0x3b);
            let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
            let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
            let set2: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            set2(member, r2);
        }
        callee_thiscall!(5, u32, this_ptr, 0)
    }
});
