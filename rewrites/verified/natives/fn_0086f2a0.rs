// original: 0x0086f2a0 VMAG
/// Script native `VMAG` (hash 0x405B02B7).
///
/// Computes the magnitude of the vector (x, y, z) as
/// `sqrt(x*x + y*y + z*z)` and stores the float result into the return slot.
/// This handler makes no engine call: it does the arithmetic itself.
///
/// The rewrite issues the same scalar SSE operations in the same lane order
/// as the original (square each component, add x²+y² first, then add z²,
/// then square root), so the result is bit-exact, including NaN and
/// infinity inputs. Declared by hand rather than through `export!` so the
/// SSE target feature can be enabled on this one function.
#[no_mangle]
#[target_feature(enable = "sse")]
pub unsafe extern "cdecl" fn rw_0086f2a0(ctx: *const u8) -> u32 {
    use core::arch::x86::*;
    let args = (*(ctx.add(8) as *const u32)) as *const f32;
    let slot = *(ctx as *const u32) as *mut f32;
    let vx = _mm_set_ss(*args);
    let vy = _mm_set_ss(*args.add(1));
    let vz = _mm_set_ss(*args.add(2));
    let xx = _mm_mul_ss(vx, vx);
    let yy = _mm_mul_ss(vy, vy);
    let zz = _mm_mul_ss(vz, vz);
    let sum = _mm_add_ss(_mm_add_ss(xx, yy), zz);
    let mag = _mm_sqrt_ss(sum);
    _mm_store_ss(slot, mag);
    slot as u32
}
