// original: 0x00890800 rage::audSound::vf0
// 00890800 rage::audSound::vf0: poll each bound slot voice until one accepts
// the param, and return its answer, or zero.
export!(thiscall, rw_00890800(this: *mut u8, param: u32) -> u32 {
    unsafe {
        if (*this.add(0x39) & 0x80) != 0 && *(this.add(6) as *const u16) == 2 {
            return 0;
        }
        if (*this.add(0x3A) & 4) == 0 {
            let stride = *global::<u32>(0x115D964);
            let base = *global::<u32>(0x115D988);
            let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
            let entry = *((row.wrapping_add(base).wrapping_add(0x6F10)) as *const u32);
            for i in 0..8u32 {
                let sel = *this.add((0x48 + i) as usize);
                if sel == 0xFF {
                    continue;
                }
                let obj = stride.wrapping_mul(sel as u32).wrapping_add(entry);
                if obj == 0 {
                    continue;
                }
                let slot = *(*(obj as *const u32) as *const u32);
                let probe: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let r = probe(obj, param);
                if r != 0 {
                    return r;
                }
            }
        }
        0
    }
});
