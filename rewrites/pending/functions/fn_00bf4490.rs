// original: 0x00bf4490 emit_vec3_a
/// Emit the header, then store three float words and tag 0x29.
export!(thiscall, rw_bf4490(this: *mut u8, a0: u32, a1: u32, f2: u32, f3: u32, f4: u32) -> u32 {
    unsafe {
        let ans: u32 = callee_thiscall!(1, u32, this as u32, a0, a1);
        *((this.add(0x10)) as *mut u32) = f2;
        *((this.add(0x14)) as *mut u32) = f3;
        *this = 0x29;
        *((this.add(0x18)) as *mut u32) = f4;
        ans
    }
});
