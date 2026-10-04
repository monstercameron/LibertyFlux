// original: 0x00b24140 attach_forward
// s08_b24140: forward an attach request, then mark the entity attached.
// thiscall/3 (a, b, c): calls the attach worker (thiscall/5) with the
// global attach context, a zero flag and the three arguments in order,
// then sets bit 0x10 on the attachment-state word at +0x1E2. Returns the
// worker's answer.
export!(thiscall, rw_b24140(this: *mut u8, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let attach: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ctx = *global::<u32>(0x12B9C8C);
        let ans = attach(this as u32, ctx, 0, a, b, c);
        let state = this.add(0x1E2) as *mut u16;
        *state |= 0x10;
        ans
    }
});
