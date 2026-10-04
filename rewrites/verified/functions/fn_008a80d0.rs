// original: 0x008a80d0 aud_polar_to_cartesian
/// Write the cosine/sine pair of a wrapped angle into a 3-word record.
///
/// Reduces (0x1c2 - arg0) modulo 0x168 with signed division, converts the
/// remainder to radians (times the degrees-to-radians constant), zeroes the
/// middle word, and stores the cosine and sine of the angle in the outer
/// words. Logically void: the leftover EAX is not a return value, so the
/// contract does not compare it. The cosine/sine helpers take their argument
/// in XMM0 on the original side; the rewrite passes the bits on the stack
/// and the stub transports them (checker documents this transport).
export!(stdcall, rw_008a80d0(arg0: u32, outp: u32) -> u32 {
    unsafe {
        let x = 0x1c2u32.wrapping_sub(arg0) as i32;
        let rem = x % 0x168;
        let out = outp as *mut u8;
        *(out.add(4) as *mut u32) = 0;
        let rad = (rem as f32) * *global::<f32>(0xfe8728);
        let c: u32 = callee_cdecl!(1, u32, rad.to_bits());
        *(out as *mut u32) = c;
        let s: u32 = callee_cdecl!(2, u32, rad.to_bits());
        *(out.add(8) as *mut u32) = s;
        0
    }
});
