// original: 0x00bf44d0 emit_vec3_b
/// Emit the header, fold two bits into +0x14, store one float, tag 0x2b.
/// Returns the low bit of a3 (EAX is reloaded after the call).
export!(thiscall, rw_bf44d0(this: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32, f4: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32, a0, a1);
        let mut cl = *this.add(0x14) & 0xFC;
        cl |= (((a2 & 1) as u8) << 1) | ((a3 & 1) as u8);
        *this = 0x2B;
        *this.add(0x14) = cl;
        *((this.add(0x10)) as *mut u32) = f4;
        // EAX is reloaded after the call and ends as the low bit of a3.
        (a3 & 1) as u32
    }
});
