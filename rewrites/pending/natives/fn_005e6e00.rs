// original: 0x005e6e00 GET_MOBILE_PHONE_ROTATION
/// Script native `GET_MOBILE_PHONE_ROTATION` (hash 0x13A83A28).
///
/// Makes no engine call. It reads three words through the first script
/// argument (used as a pointer), appends bookkeeping entries to the call
/// context at offsets derived from the counter at `ctx+0xC`, copies four
/// global words (the phone rotation) into the context, bumps the counter,
/// and returns the new counter value.
export!(cdecl, rw_005e6e00(ctx: *mut u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let p = *args as *const u32;
        let w1 = *p.add(1);
        let w2 = *p.add(2);
        let count = *(ctx.add(0x0C) as *const u32);
        *(ctx.add(0x10).add(count.wrapping_mul(4) as usize) as *mut u32) = p as u32;
        let w0 = *p;
        let row = ctx.add(count.wrapping_add(2).wrapping_mul(2) as usize * 8);
        *(row as *mut u32) = w0;
        *(row.add(4) as *mut u32) = w1;
        *(row.add(8) as *mut u32) = w2;
        let out = ctx.add(count.wrapping_add(2) as usize * 16);
        *(ctx.add(0x0C) as *mut u32) = count.wrapping_add(1);
        *(out as *mut u32) = *global::<u32>(0x19D3B30);
        *(out.add(4) as *mut u32) = *global::<u32>(0x19D3B34);
        *(out.add(8) as *mut u32) = *global::<u32>(0x19D3B38);
        *(out.add(12) as *mut u32) = *global::<u32>(0x19D3B3C);
        count.wrapping_add(1)
    }
});
