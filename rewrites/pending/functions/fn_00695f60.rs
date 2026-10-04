// original: 0x00695f60 channel_group_sizer
/// Measure the serialised size of a channel-group list.
///
/// Walks every group and sums each member's virtual size contribution plus
/// fixed per-group overhead. Returns the total.
export!(thiscall, rs80_695f60(this: *const u8) -> u32 {
    unsafe {
        const BASE: u32 = 0x10;
        const GROUP_OVERHEAD: u32 = 0x18;
        const GROUP_STEP: u32 = 4;
        let outer = *((this).add(0x0C) as *const u16) as u32;
        let groups = *((this).add(8) as *const u32) as *const u32;
        let mut total = BASE;
        for g in 0..outer {
            let ebp = *groups.add(g as usize) as *const u8;
            let inner = *(ebp.add(0x14) as *const i32);
            let mut sub = GROUP_OVERHEAD;
            if inner > 0 {
                let arr = ebp.add(4) as *const u32;
                for i in 0..(inner as usize) {
                    let o = *arr.add(i);
                    let vtbl = *((o as *const u8).add(0) as *const u32) as *const u8;
                    sub = sub.wrapping_add(({
        let __a = *((vtbl.add(0x4C)) as *const u32);
        let __f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(__a as usize);
        __f(o)
    }));
                }
            }
            total = total.wrapping_add(GROUP_STEP).wrapping_add(sub);
        }
        total
    }
});
