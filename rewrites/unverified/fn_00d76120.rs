// original: 0x00D76120 blend_float_pairs_down (proposed)

/// Blend two float pairs and hand the sums down with a marker.
///
/// Halves both floats of `q`, adds them onto the floats of `p`, and
/// passes the two sums in a frame block (preceded by the 0xffff0000
/// marker word) to the next stage along with the two halves by value,
/// whose answer is returned. Stdcall, two pointer words. Float operation
/// order is the original's.
use lf_checker_rt::{callee_stdcall, export};

const NEXT: u32 = 1;

export!(stdcall, rw_00d76120(p: u32, q: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        const HALF: f32 = f32::from_bits(0x3f00_0000); // 0.5
        const MARKER: u32 = 0xffff_0000;
        let h0 = mul(rdf(q), HALF);
        let h1 = mul(rdf(q + 4), HALF);
        let s0 = add(rdf(p), h0);
        let s1 = add(rdf(p + 4), h1);
        let mut blk = [MARKER, s0.to_bits(), s1.to_bits()];
        let base = blk.as_mut_ptr() as u32;
        callee_stdcall!(NEXT, u32, base + 4, h0.to_bits(), h1.to_bits(), base)
    }
});
