// original: 0x00b653d0 store_and_clamped_apply
// thiscall/2. Resets when the pending byte is set, stores the first argument,
// then applies the second argument's low half clamped to 0x61a8. Returns nothing.
export!(thiscall, rw_rs11f16(this: *mut u8, a: u32, b: u32) -> () {
    unsafe {
        if *this.add(8) != 0 {
            let reset: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            reset(this as u32);
        }
        *(this as *mut u32) = a;
        let clamped = (b & 0xffff).min(0x61a8);
        *this.add(8) = 0;
        let apply: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        apply(this as u32, clamped);
    }
});
