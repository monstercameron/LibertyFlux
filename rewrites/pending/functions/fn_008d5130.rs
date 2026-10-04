// original: 0x008d5130 angle_wrap_once
/// Single-step angle wrap: subtracts 2*pi once when above pi, adds 2*pi once
/// when below -pi, else unchanged. NaN passes through untouched.
export!(cdecl, rw_008d5130(x: f32) -> f32 {
    unsafe {
        let pi = *global::<f32>(0x00FE_8AA0);
        let two_pi = *global::<f32>(0x00FE_8AEC);
        let mpi = *global::<f32>(0x00FE_8DC4);
        if x > pi {
            fsub(x, two_pi)
        } else if mpi > x {
            fadd(x, two_pi)
        } else {
            x
        }
    }
});
