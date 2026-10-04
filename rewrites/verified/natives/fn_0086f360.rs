// original: 0x0086f360 VDIST2
use lf_k2_rt::{export};
/// Native handler `VDIST2`.
///
/// Returns the squared distance between two 3D points.
///
/// Handler mechanics: takes the native call context,
/// Pure computation over six float arguments; no engine call. The
/// accumulation order matches the original exactly (bit-exact).
export!(cdecl, rw_0086f360(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let x0 = f32::from_bits(unsafe { *args });
    let y0 = f32::from_bits(unsafe { *args.add(1) });
    let z0 = f32::from_bits(unsafe { *args.add(2) });
    let x1 = f32::from_bits(unsafe { *args.add(3) });
    let y1 = f32::from_bits(unsafe { *args.add(4) });
    let z1 = f32::from_bits(unsafe { *args.add(5) });
    let dx = x0 - x1;
    let dy = y0 - y1;
    let dz = z0 - z1;
    let d2 = dy * dy + dx * dx + dz * dz;
    unsafe { *(ret as *mut u32) = d2.to_bits() };
});
