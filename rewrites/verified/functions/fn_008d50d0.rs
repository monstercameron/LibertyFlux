// original: 0x008d50d0 angle_wrap_loop
/// Wraps an angle into [-pi, pi] by repeated subtraction/addition of 2*pi.
/// NaN passes through untouched (unordered comparisons skip both loops).
export!(cdecl, rw_008d50d0(x: f32) -> f32 {
    unsafe {
        let mut x = x;
        let pi = *global::<f32>(0x00FE_8AA0);
        let two_pi = *global::<f32>(0x00FE_8AEC);
        let mpi = *global::<f32>(0x00FE_8DC4);
        if x > pi {
            loop {
                x = fsub(x, two_pi);
                if !(x > pi) {
                    break;
                }
            }
        }
        if mpi > x {
            loop {
                x = fadd(x, two_pi);
                if !(mpi > x) {
                    break;
                }
            }
        }
        x
    }
});
