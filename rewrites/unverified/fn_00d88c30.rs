// original: 0x00d88c30 proposed_solver_probe_wrap
// Solver probe: stages a descriptor from a 4-float source vector, two
// scalar helpers and three staged constants, dispatches it through the
// solver object, wraps an angle into [0, TAU], and reports the probe's
// status word. `p0` points at the 4 source floats; `f1`/`f4` feed the
// scalar helpers; `f2` scales their answers into the descriptor; `f3`
// offsets the wrapped angle. Returns the output pointer with its low
// byte replaced by the status (1 = idle, 0 = probe hit, with the hit
// distance stored at `out2`).

use core::f32::consts::TAU;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

export!(cdecl, rw_d88c30(p0: u32, f1: f32, f2: f32, f3: f32, f4: f32, out1: u32, out2: u32) -> u32 {
    unsafe {
        let _staged = (
            global::<f32>(0x1b4b320).read(),
            global::<f32>(0x1b4b324).read(),
            global::<f32>(0x1b4b328).read(),
        );
        let s0 = ((p0 + 0x0) as *const f32).read();
        let s1 = ((p0 + 0x4) as *const f32).read();
        let s2 = ((p0 + 0x8) as *const f32).read();
        let s3 = ((p0 + 0xc) as *const f32).read();
        let x = f1 + f4;
        let ans1 = f32::from_bits(callee_cdecl!(1, u32, x.to_bits()));
        let ans2 = f32::from_bits(callee_cdecl!(2, u32, x.to_bits()));
        let h = global::<u32>(0x12b9c78).read();
        let q0 = ans1 * f2 + s0;
        let q1 = ans2 * f2 + s1;
        let q2 = f2 * -0.0 + s2;
        let inbuf = [
            s0.to_bits(), s1.to_bits(), s2.to_bits(), s3.to_bits(),
            q0.to_bits(), q1.to_bits(), q2.to_bits(), s3.to_bits(),
        ];
        let mut outbuf = [0u32; 6];
        callee_thiscall!(3, u32, h, inbuf.as_ptr() as u32, outbuf.as_mut_ptr() as u32,
            0, 6, 0xFFFFFFFF, 7, 1, 0);
        let mut ang = f1 + f3;
        (out1 as *mut f32).write(ang);
        while 0.0 > ang {
            ang += TAU;
        }
        (out1 as *mut f32).write(ang);
        ang = (out1 as *const f32).read();
        while ang > TAU {
            ang -= TAU;
        }
        (out1 as *mut f32).write(ang);
        if outbuf[0] == 0 {
            (out1 & 0xFFFFFF00) | 1
        } else {
            let dx = f32::from_bits(outbuf[5]) - s1;
            let dy = f32::from_bits(outbuf[4]) - s0;
            (out2 as *mut f32).write((dx * dx + dy * dy).sqrt());
            out2 & 0xFFFFFF00
        }
    }
});
