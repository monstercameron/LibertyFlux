// original: 0x0088F030 rage::audVoicePcAdpcm::vf10
// 0088F030 rage::audVoicePcAdpcm::vf10: poll the voice hook; when it is
// silent return -1, otherwise mix the helper outputs for the live path
// (flag 0x10) or the starting path.
export!(thiscall, rw_0088f030(this: *mut u8) -> u32 {
    unsafe {
        let vtable = *(this as *const u32);
        let target = *((vtable as *const u8).add(0x18) as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if probe(this as u32) & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let child = *(this.add(0x140) as *const u32);
        if (*this.add(0x8C) & 0x10) != 0 {
            let have = callee_thiscall!(2, u32, child);
            let mixer = *global::<u32>(0x115A448);
            let base = callee_thiscall!(3, u32, mixer);
            let extra = if have < base {
                0
            } else {
                have.wrapping_sub(base)
            };
            let first = callee_cdecl!(
                4,
                u32,
                *(this.add(0x128) as *const u32),
                *(this.add(0xC) as *const u32)
            );
            let second = callee_cdecl!(
                4,
                u32,
                extra,
                *((mixer.wrapping_add(0x80)) as *const u32)
            );
            first.wrapping_add(second)
        } else {
            let tick = callee_thiscall!(5, u32, child);
            let span = *(this.add(0x124) as *const u32);
            let pos = ((span >> 1) << 17)
                .wrapping_add(tick)
                .wrapping_shr(1)
                .wrapping_add(*(this.add(0x128) as *const u32));
            callee_cdecl!(4, u32, pos, *(this.add(0xC) as *const u32))
        }
    }
});
