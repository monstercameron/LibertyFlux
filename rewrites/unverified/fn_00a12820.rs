// original: 0x00a12820 speed_flag_update (proposed)
/// Set or clear the speed flag from the object's velocity vector.
///
/// Reads the target object at `this + 0x12c`. When `arg` is null or its
/// word at `+0x1304` is not 1, clears bit 0 at target `+0x1481`. Otherwise
/// fetches the velocity vector through the virtual slot at `+0xec` of `arg`
/// and sets the bit when the vector's length is strictly above the constant
/// 0x3e99999a, else clears it. Returns the target object. Thiscall, one
/// stack argument.
export!(thiscall, rw_00a12820(this: u32, arg: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 10;
        const OBJ_OFF: u32 = 0x12c;
        const KIND_OFF: u32 = 0x1304;
        const VTABLE_SLOT: u32 = 0xec;
        const FLAG_OFF: u32 = 0x1481;
        const LIMIT_ADDR: u32 = 0x00fe87e8;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let obj = ((this + OBJ_OFF) as *const u32).read_unaligned();
        let mut set = false;
        if arg != 0 && ((arg + KIND_OFF) as *const u32).read_unaligned() == 1 {
            let vt = (arg as *const u32).read_unaligned();
            let slot = ((vt + VTABLE_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let mut scratch = [0u32; 4];
            let v = f(arg, &mut scratch as *mut u32 as u32);
            let x = f32::from_bits((v as *const u32).read_unaligned());
            let y = f32::from_bits(((v + 4) as *const u32).read_unaligned());
            let z = f32::from_bits(((v + 8) as *const u32).read_unaligned());
            let d = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let len = d.sqrt();
            set = len > f32::from_bits(*global::<u32>(LIMIT_ADDR));
        }
        let fb = (obj + FLAG_OFF) as *mut u8;
        if set {
            fb.write(fb.read() | 1);
        } else {
            fb.write(fb.read() & 0xfe);
        }
        obj
    }
});
