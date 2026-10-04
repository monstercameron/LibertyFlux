// original: 0x0088F3A0 
// 0088F3A0 audVoice gain update: a zero gain takes the flag path (latch the
// live bit from the child), any other gain scales the stored count and
// programs the child. The float-to-int step truncates toward zero and yields
// zero whenever the value is not a representable integer.
export!(thiscall, rw_0088f3a0(this: *mut u8, arg_bits: u32) -> () {
    unsafe {
        let arg = f32::from_bits(arg_bits);
        if arg != *global::<f32>(0xFE8628) {
            let raw = *(this.add(0xC) as *const u32);
            let bias = *global::<f64>(0xFE8F50 + (raw >> 31) * 8);
            let scaled = ((raw as i32 as f64) + bias) as f32 * arg;
            const LIM: f32 = 9.223372036854776e18;
            let chopped = if scaled.is_nan() || !(scaled > -LIM && scaled < LIM) {
                0u32
            } else {
                scaled.trunc() as i64 as u32
            };
            *(this.add(0x13C) as *mut u32) = chopped;
            callee_thiscall!(3, u32, *(this.add(0x140) as *const u32), chopped);
            let flags = *this.add(0x8C);
            if flags & 1 != 0 {
                if flags & 0x40 != 0 {
                    *this.add(0x8C) = flags | 8;
                }
                *this.add(0x8C) &= 0xFE;
            }
        } else {
            if (*this.add(0x8C) & 1) != 0 {
                return;
            }
            let child = *(this.add(0x140) as *const u32);
            let live = child != 0 && callee_thiscall!(1, u32, child) & 0xFF != 0;
            *this.add(0x8C) = (*this.add(0x8C) & !0x40) | ((live as u8) << 6);
            if live {
                callee_thiscall!(2, u32, child);
            }
            *this.add(0x8C) |= 1;
        }
    }
});
