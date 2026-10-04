// original: 0x009252e0 view_frustum_update (proposed)

/// Recompute the view frustum from two queried extents, optionally bracketed
/// by device suspend/resume calls.
///
/// `obj` is an object with a vtable; `tag` is an opaque word forwarded to a
/// helper; only the low byte of `flags` matters. When `flags & 0xFF` is
/// nonzero, the device object at 0x17F5630 is first told to suspend (vtable
/// slot +0x3C: thiscall with `0, obj, 0, 0, 1, -1`). Then the helper at its
/// slot (callee 2: thiscall on 0x119CFF8 with `[0x119D070]` and `tag`) runs,
/// and two extent queries are made on `obj` (slots +0x20 and +0x24, thiscall,
/// no stack args), each returning an integer count. Each count is converted
/// to float and divided into 1.0 (the image constant at 0xFE88E8); either
/// quotient may be infinite when its count is zero. The first quotient is
/// spilled to the incoming `flags` stack slot; both are then halved (image
/// constant 0.5 at 0xFE8830) and passed, together with a fixed five-word
/// header (-1, 1, 1, -1, 1), their plus-one values, -1 and `[0x119D07C]`, as
/// eleven stack words to the frustum writer (callee 5: thiscall on
/// 0x119D000). When the suspend ran, the device is resumed afterwards
/// (slot +0x40: thiscall with `0, 0, -1`). Returns the resume call's answer
/// when it ran, else the frustum writer's. Float order is the original's,
/// pinned through black-boxed operands.
///
/// Original: 0x009252e0 (cdecl, three stack words, plain `ret`).
lf_checker_rt::export!(cdecl, rw_009252e0(obj: u32, tag: u32, flags: u32) -> u32 {
    unsafe {
        const DEVICE: u32 = 0x17F5630;
        const ONE_ADDR: u32 = 0x00FE88E8;
        const HALF_ADDR: u32 = 0x00FE8830;

        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let suspend = (flags & 0xFF) as u8 != 0;
        if suspend {
            let dev = lf_checker_rt::global::<u32>(DEVICE).read_unaligned();
            let vtable = (dev as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute((vtable.wrapping_add(0x3C) as *const u32).read_unaligned() as usize);
            f(dev, 0, obj, 0, 0, 1, 0xFFFFFFFF);
        }
        let helper_this = lf_checker_rt::global::<u32>(0x119CFF8).read_unaligned();
        let helper_arg = lf_checker_rt::global::<u32>(0x119D070).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, helper_this, helper_arg, tag);
        let vtable = (obj as *const u32).read_unaligned();
        let q0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtable.wrapping_add(0x20) as *const u32).read_unaligned() as usize);
        let q1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtable.wrapping_add(0x24) as *const u32).read_unaligned() as usize);
        let one = (lf_checker_rt::global::<u32>(ONE_ADDR) as *const f32).read_unaligned();
        let half = (lf_checker_rt::global::<u32>(HALF_ADDR) as *const f32).read_unaligned();
        let n0 = q0(obj) as i32;
        let mut x = div(one, n0 as f32);
        let n1 = q1(obj) as i32;
        let mut y = div(one, n1 as f32);
        x = mul(x, half);
        y = mul(y, half);
        let tail = lf_checker_rt::global::<u32>(0x119D07C).read_unaligned();
        let writer_this = lf_checker_rt::global::<u32>(0x119D000).read_unaligned();
        let r = lf_checker_rt::callee_thiscall!(
            5, u32, writer_this,
            0xBF800000u32, 0x3F800000, 0x3F800000, 0xBF800000, 0x3F800000,
            x.to_bits(), y.to_bits(), add(x, one).to_bits(), add(y, one).to_bits(),
            0xFFFFFFFFu32, tail
        );
        if suspend {
            let dev = lf_checker_rt::global::<u32>(DEVICE).read_unaligned();
            let vtable = (dev as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute((vtable.wrapping_add(0x40) as *const u32).read_unaligned() as usize);
            f(dev, 0, 0, 0xFFFFFFFF)
        } else {
            r
        }
    }
});
