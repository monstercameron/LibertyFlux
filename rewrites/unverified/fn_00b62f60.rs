// original: 0x00b62f60 veh_resolve_scale_float
/// Resolve a handle to a float, publish it, and store a scaled product.
///
/// Looks up `a0` through a resolved handle: the first call maps
/// `[this+0x18]` to a manager, the second converts `a0` to a float result.
/// Writes 0 to `[a2]` and the float to `[a3]`, then asks a third call for an
/// integer count, converts it to float, multiplies it by a
/// constant from the image and then by the converted float (in that order),
/// and stores the product to `[a1]`. Returns `a1`.
///
/// The float result is also spilled into the incoming `a0` stack slot; the
/// stack comparison is off for that reason (see narrowed).
///
/// Original: thiscall, ECX plus four stack words.
lf_checker_rt::export!(thiscall, rw_00b62f60(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x18;
        const RESOLVE: u32 = 1;
        const CONVERT: u32 = 2;
        const COUNT: u32 = 3;
        const SCALE_VA: u32 = 0x00fe8684;
        let key = ((this + KEY_OFF) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, key);
        let f: f32 = lf_checker_rt::callee_thiscall!(CONVERT, f32, h, a0);
        (a2 as *mut u32).write_unaligned(0);
        (a3 as *mut u32).write_unaligned(f.to_bits());
        let n: u32 = lf_checker_rt::callee_stdcall!(COUNT, u32,);
        let scale = f32::from_bits(lf_checker_rt::global::<u32>(SCALE_VA).read_unaligned());
        let x = (n as i32) as f32;
        let y = core::hint::black_box(x) * core::hint::black_box(scale);
        let z = core::hint::black_box(y) * core::hint::black_box(f);
        (a1 as *mut u32).write_unaligned(z.to_bits());
        a1
    }
});

/// Wrong version of rw_00b62f60: stores 1 instead of 0 to `[a2]`.
lf_checker_rt::export!(thiscall, mut_00b62f60(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x18;
        const RESOLVE: u32 = 1;
        const CONVERT: u32 = 2;
        const COUNT: u32 = 3;
        const SCALE_VA: u32 = 0x00fe8684;
        let key = ((this + KEY_OFF) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, key);
        let f: f32 = lf_checker_rt::callee_thiscall!(CONVERT, f32, h, a0);
        // MUTANT: 1 instead of 0.
        (a2 as *mut u32).write_unaligned(1);
        (a3 as *mut u32).write_unaligned(f.to_bits());
        let n: u32 = lf_checker_rt::callee_stdcall!(COUNT, u32,);
        let scale = f32::from_bits(lf_checker_rt::global::<u32>(SCALE_VA).read_unaligned());
        let x = (n as i32) as f32;
        let y = core::hint::black_box(x) * core::hint::black_box(scale);
        let z = core::hint::black_box(y) * core::hint::black_box(f);
        (a1 as *mut u32).write_unaligned(z.to_bits());
        a1
    }
});
