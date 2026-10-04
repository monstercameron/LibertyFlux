// original: 0x00890870 rage::audSound::vf1
// 00890870 rage::audSound::vf1: map the param through the helper, then
// forward it to the primary voice hook.
export!(thiscall, rw_00890870(this: *mut u8, param: u32) -> u32 {
    unsafe {
        let v = callee_cdecl!(1, u32, param, 0);
        let target = *(*(this as *const u32) as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        hook(this as u32, v)
    }
});
