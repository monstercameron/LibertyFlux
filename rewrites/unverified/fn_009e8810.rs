// original: 0x009e8810 ped_pose_or_copy
/// Poses the object when its motion sample at `+0x7B4` is live and
/// its sample vector (floats at `+0x40/0x44/0x48`) has squared length
/// above 1.0: slot `index` of the pose table is resolved and the two
/// pose calls run, returning 1. Otherwise twelve words are copied from
/// `[this+0x20]` to the out-pointer (skipping `+0xC` and `+0x1C`) and 0
/// is returned. (thiscall, 2 args; the low byte is the decision.)
lf_checker_rt::export!(thiscall, rw_009e8810(this_ptr: u32, out_ptr: u32, index: u32) -> u32 {
    unsafe {
        const SAMPLE_OFF: u32 = 0x7B4;
        const FLAG_OFF: u32 = 0x64;
        const VEC_Y_OFF: u32 = 0x40;
        const VEC_X_OFF: u32 = 0x44;
        const VEC_Z_OFF: u32 = 0x48;
        const INNER_OFF: u32 = 4;
        const TABLE_OFF: u32 = 0xC;
        const SLOT_OFF: u32 = 0x84;
        const COPY_SRC_OFF: u32 = 0x20;
        const THRESH: u32 = 0xFE88E8;
        const COPY_WORDS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let obj = (this_ptr.wrapping_add(SAMPLE_OFF) as *const u32).read_unaligned();
        let mut live = obj != 0;
        if live {
            live = (obj.wrapping_add(FLAG_OFF) as *const u32).read_unaligned() != 0;
        }
        if live {
            let y = f32::from_bits((obj.wrapping_add(VEC_Y_OFF) as *const u32).read_unaligned());
            let x = f32::from_bits((obj.wrapping_add(VEC_X_OFF) as *const u32).read_unaligned());
            let z = f32::from_bits((obj.wrapping_add(VEC_Z_OFF) as *const u32).read_unaligned());
            let s = add(add(mul(y, y), mul(x, x)), mul(z, z));
            let t = f32::from_bits(lf_checker_rt::global::<u32>(THRESH).read_unaligned());
            live = s > t;
        }
        if live {
            let inner = (obj.wrapping_add(INNER_OFF) as *const u32).read_unaligned();
            let table = (inner.wrapping_add(TABLE_OFF) as *const u32).read_unaligned();
            let slot = index.wrapping_shl(6)
                .wrapping_add((table.wrapping_add(SLOT_OFF) as *const u32).read_unaligned());
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, out_ptr, slot);
            let obj2 = (this_ptr.wrapping_add(SAMPLE_OFF) as *const u32).read_unaligned();
            let ans: u32 = lf_checker_rt::callee_thiscall!(2, u32, out_ptr, obj2.wrapping_add(0x10));
            (ans & 0xFFFFFF00) | 1
        } else {
            let src = (this_ptr.wrapping_add(COPY_SRC_OFF) as *const u32).read_unaligned();
            for off in COPY_WORDS {
                let w = (src.wrapping_add(off) as *const u32).read_unaligned();
                (out_ptr.wrapping_add(off) as *mut u32).write_unaligned(w);
            }
            let last = (src.wrapping_add(0x38) as *const u32).read_unaligned();
            last & 0xFFFFFF00
        }
    }
});
