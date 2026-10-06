// original: 0x005d5550 viewport_stage_submit (proposed)
//
// Submit one viewport stage: resolve the render target through a dispatch
// table, upload a twelve-float parameter block, and run two lined passes.
//
// The viewport object comes from a global (`ebx`); the stack argument is
// ignored. A getter callee on its vtable slot `+0xa0` either yields the
// tag object directly (through a second vtable slot `+0xe0` call) or falls
// back to the word at `+0x100`. A tag callee consumes the tag's `+4` word
// with 0x4d0, and its answer feeds the setup callee; then twelve floats
// are copied from the `+0x20` block into a stack array (at the original's
// exact word offsets, with the three holes it never stores left as the
// zero fill both sides observe). An upload callee then takes the row object at
// `+0x2c4`, the setup answer, the array pointer and 1. The pass object is
// selected from a global pointer table by the signed halfword at row
// `+0x2e`: a null `+8` link follows `+0xc` and dereferences once more,
// otherwise `+0xb4` past the link wins. A byte-scaled float (the `+0x5b`
// byte times a global constant) and a flag callee's low byte go to two
// scalar callees, a state callee takes the `+0x34` object with 0, and two
// line passes run the pass object's vtable slot `+0x20` with the array
// pointer, the pass index and two zeroes. A second state call and two
// scalar calls with 1.0 and 0 finish.
//
// Original: 0x005d5550 (stdcall, one ignored stack word; returns the last
// scalar callee's answer).
lf_checker_rt::export!(stdcall, rw_005d5550(_unused: u32) -> u32 {
    unsafe {
        const ID_TAG: u32 = 3;
        const ID_SETUP: u32 = 4;
        const ID_UPLOAD: u32 = 5;
        const ID_SCALAR1: u32 = 6;
        const ID_FLAG: u32 = 7;
        const ID_SCALAR2: u32 = 8;
        const ID_STATE1: u32 = 9;
        const ID_STATE2: u32 = 11;
        const ONE_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let ebx = rd32(lf_checker_rt::relocated(0x018b6ebc));
        let get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(ebx).wrapping_add(0xa0)) as usize);
        let tag = if get(ebx) != 0 {
            let again = get(ebx);
            let use_it: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(again).wrapping_add(0xe0)) as usize);
            use_it(again)
        } else {
            rd32(ebx.wrapping_add(0x100))
        };
        let tagans: u32 = lf_checker_rt::callee_cdecl!(ID_TAG, u32, rd32(tag.wrapping_add(4)), 0x4d0);
        let src = rd32(ebx.wrapping_add(0x20));
        // The original's exact stack layout: twelve stored words with holes
        // at indices 3, 7 and 11, which neither side stores (zero fill).
        let mut arr = [0u32; 15];
        arr[0] = rd32(src);
        arr[1] = rd32(src.wrapping_add(4));
        arr[2] = rd32(src.wrapping_add(8));
        arr[4] = rd32(src.wrapping_add(0x10));
        arr[5] = rd32(src.wrapping_add(0x14));
        arr[6] = rd32(src.wrapping_add(0x18));
        arr[8] = rd32(src.wrapping_add(0x20));
        arr[9] = rd32(src.wrapping_add(0x24));
        arr[10] = rd32(src.wrapping_add(0x28));
        arr[12] = rd32(src.wrapping_add(0x30));
        arr[13] = rd32(src.wrapping_add(0x34));
        arr[14] = rd32(src.wrapping_add(0x38));
        let setup: u32 = lf_checker_rt::callee_thiscall!(ID_SETUP, u32, ebx, tagans);
        let row = rd32(ebx.wrapping_add(0x2c4));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ID_UPLOAD, u32, rd32(row.wrapping_add(0x25c)), row, setup, arr.as_mut_ptr() as u32, 1
        );
        let idx = ((row.wrapping_add(0x2e)) as *const i16).read_unaligned() as i32;
        let entry = rd32(
            lf_checker_rt::relocated(0x01295cd8)
                .wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let link = rd32(entry.wrapping_add(8));
        let pass = if link != 0 {
            rd32(link.wrapping_add(0xb4))
        } else {
            let c = rd32(entry.wrapping_add(0xc));
            if c != 0 {
                rd32(c)
            } else {
                0
            }
        };
        let scaled = mul(
            core::hint::black_box(rd8(ebx.wrapping_add(0x5b)) as f32),
            f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe86e8))),
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(ID_SCALAR1, u32, scaled.to_bits());
        let flag: u32 = lf_checker_rt::callee_thiscall!(ID_FLAG, u32, ebx);
        let _: u32 = lf_checker_rt::callee_cdecl!(ID_SCALAR2, u32, flag & 0xff);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_STATE1, u32, rd32(ebx.wrapping_add(0x34)), 0);
        for si in 0..2u32 {
            let line: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(pass).wrapping_add(0x20)) as usize);
            let _: u32 = line(pass, arr.as_mut_ptr() as u32, si, 0, 0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_STATE2, u32, rd32(ebx.wrapping_add(0x34)), 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(ID_SCALAR1, u32, ONE_BITS);
        lf_checker_rt::callee_cdecl!(ID_SCALAR2, u32, 0)
    }
});
