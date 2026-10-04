// original: 0x00dd9570 UIClip::vf107
/// `UIClip::vf107`: apply the mode selected by `flag_321` with a gate.
///
/// Like the montage variant, with the helper constant 2 in mode 2 and `0x3e`
/// otherwise, the `field_1e4` post-select running in modes 1 and 2, and an
/// extra gated block: when the `field_1f0` member's slot `0x124` check passes,
/// the `field_1ec` member's slot `0x208` runs with a second helper value.
/// Finishes through the shared helper and returns its answer.
export!(thiscall, rw_00dd9570(this_ptr: u32) -> u32 {
    unsafe {
        let flag = ((this_ptr as *const u8).add(0x321)).read();
        let k: u32 = if flag == 2 { 2 } else { 0x3e };
        let scratch = 0u32;
        let fp = &scratch as *const u32 as u32;
        let r = callee_cdecl!(0, u32, fp, k);
        let member = ((this_ptr as *const u32).add(0x1e8 / 4)).read();
        let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
        let set: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        set(member, r);
        if flag == 1 || flag == 2 {
            let member = ((this_ptr as *const u32).add(0x1e4 / 4)).read();
            let slot = (((member as *const u32).read() + 0x120) as *const u32).read();
            let post: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            post(member, 1);
        }
        let member = ((this_ptr as *const u32).add(0x1f0 / 4)).read();
        let slot = (((member as *const u32).read() + 0x124) as *const u32).read();
        let gate: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(slot as usize);
        if gate(member) != 0 {
            let r2 = callee_cdecl!(0, u32, fp, k);
            let member = ((this_ptr as *const u32).add(0x1ec / 4)).read();
            let slot = (((member as *const u32).read() + 0x208) as *const u32).read();
            let set2: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            set2(member, r2);
        }
        callee_thiscall!(5, u32, this_ptr, 1)
    }
});
