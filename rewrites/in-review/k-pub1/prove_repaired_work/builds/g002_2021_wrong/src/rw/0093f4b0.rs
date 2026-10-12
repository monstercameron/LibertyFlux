lf_checker_rt::export!(cdecl, rw_0093f4b0(list: u32, bx: u32, func: u32, ctx: u32) -> u32 {
    // First test not strict: >= instead of >.
    unsafe {
        let mut node = (list as *const u32).read_unaligned();
        if node == 0 {
            return 1;
        }
        let gen = (lf_checker_rt::global::<u16>(0x11A8908) as *const u16).read_unaligned() as u32;
        let blo = [
            (bx as *const f32).read_unaligned(),
            ((bx + 4) as *const f32).read_unaligned(),
            ((bx + 8) as *const f32).read_unaligned(),
        ];
        let bhi = [
            ((bx + 0x10) as *const f32).read_unaligned(),
            ((bx + 0x14) as *const f32).read_unaligned(),
            ((bx + 0x18) as *const f32).read_unaligned(),
        ];
        loop {
            let obj = (node as *const u32).read_unaligned();
            node = ((node + 4) as *const u32).read_unaligned();
            if ((obj + 0x3C) as *const u32).read_unaligned() != gen {
                ((obj + 0x3C) as *mut u32).write_unaligned(gen);
                let point: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + 0x50) as *const u32)
                        .read_unaligned()) as usize,
                );
                let mut frame = [0u32; 3];
                point(obj, frame.as_mut_ptr() as u32);
                let p = [
                    f32::from_bits(frame[0]),
                    f32::from_bits(frame[1]),
                    f32::from_bits(frame[2]),
                ];
                let inside = p[0] >= blo[0] && p[1] > blo[1] && p[2] > blo[2]
                    && bhi[0] > p[0] && bhi[1] > p[1] && bhi[2] > p[2];
                if inside {
                    let f: extern "cdecl" fn(u32, u32) -> u32 =
                        core::mem::transmute(func as usize);
                    if (f(obj, ctx) & 0xFF) == 0 {
                        return 0;
                    }
                }
            }
            if node == 0 {
                break;
            }
        }
        1
    }
});
