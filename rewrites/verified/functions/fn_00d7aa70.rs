// original: 0x00D7AA70 probe_vector_falloff (proposed)

/// Probe a vector through a virtual helper and map its length to a factor.
///
/// Calls virtual slot `0xec` of the object at `*a0` with an out-pointer,
/// then takes the returned pointer's three floats, forms the length
/// `sqrt(x*x + y*y + z*z)`, and returns in ST0: 0.2 when the length is
/// strictly above 42.0, else 0.7 when `0.9 - length * 0.016666668` is
/// strictly above 0.7, else that difference itself. Cdecl, one stack
/// word. Float operation order is the original's.
use lf_checker_rt::export;

export!(cdecl, rw_00d7aa70(a0: u32) -> f32 {
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        const VSLOT: u32 = 0xec;
        const LEN_LIMIT: f32 = f32::from_bits(0x4228_0000); // 42.0
        const FLAT: f32 = f32::from_bits(0x3e4c_cccd); // 0.2
        const SLOPE: f32 = f32::from_bits(0x3c88_8889); // 0.016666668
        const TOP: f32 = f32::from_bits(0x3f66_6666); // 0.9
        const CAP: f32 = f32::from_bits(0x3f33_3333); // 0.7
        let vtable = (a0 as *const u32).read_unaligned();
        let slot = ((vtable + VSLOT) as *const u32).read_unaligned();
        let mut scratch = [0u32; 4];
        let probe: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let ans = probe(a0, scratch.as_mut_ptr() as u32);
        let x = f32::from_bits((ans as *const u32).read_unaligned());
        let y = f32::from_bits(((ans + 4) as *const u32).read_unaligned());
        let z = f32::from_bits(((ans + 8) as *const u32).read_unaligned());
        let len = add(add(mul(x, x), mul(y, y)), mul(z, z)).sqrt();
        if len > LEN_LIMIT {
            return FLAT;
        }
        let w = sub(TOP, mul(len, SLOPE));
        if w > CAP {
            CAP
        } else {
            w
        }
    }
});
