// original: 0x0093f860 stream_nodes_in_dilated_box (proposed)

/// Call `func` for each unvisited node whose center is in the dilated box.
///
/// Walks the node list at `list` (object at +0, next at +4), skipping
/// nodes already stamped with the epoch at `EPOCH` and stamping the rest.
/// Each node's radius comes from virtual slot `RADIUS_SLOT` (an x87
/// single, read here through the callee's integer channel, which carries
/// the same bits) and its center from slot `CENTER_SLOT` into scratch.
/// `func(obj, ctx)` runs when the center is strictly inside the box
/// dilated by the radius (`box[i] - r < c[i] < box[i+4] + r`, all three).
/// A zero answer stops the walk with 0, exhaustion returns 1. Strict
/// greater-than throughout, so NaN never passes. Only `al` is set.
///
/// Original: 0x0093f860 (cdecl, four stack words; two virtual + one
/// register callee).
lf_checker_rt::export!(cdecl, rw_0093f860(list: u32, bx: u32, func: u32, ctx: u32) -> u32 {
    const EPOCH: u32 = 0x11A8908;
    const OBJ_STAMP: u32 = 0x3C;
    const RADIUS_SLOT: u32 = 0x58;
    const CENTER_SLOT: u32 = 0x50;
    unsafe {
        let mut node = (list as *const u32).read_unaligned();
        if node == 0 {
            return 1;
        }
        let gen = (lf_checker_rt::global::<u16>(EPOCH) as *const u16).read_unaligned() as u32;
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
            if ((obj + OBJ_STAMP) as *const u32).read_unaligned() != gen {
                let radius: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + RADIUS_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let r = f32::from_bits(core::hint::black_box(radius(obj)));
                ((obj + OBJ_STAMP) as *mut u32).write_unaligned(gen);
                let center: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + CENTER_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let mut frame = [0u32; 3];
                center(obj, frame.as_mut_ptr() as u32);
                let c = [
                    f32::from_bits(core::hint::black_box(frame[0])),
                    f32::from_bits(core::hint::black_box(frame[1])),
                    f32::from_bits(core::hint::black_box(frame[2])),
                ];
                // Operand order pinned: the original computes box-r and
                // box+r per lane with the box word first.
                let bb = core::hint::black_box;
                let inside = bb(c[0]) > bb(blo[0]) - bb(r)
                    && bb(c[1]) > bb(blo[1]) - bb(r)
                    && bb(c[2]) > bb(blo[2]) - bb(r)
                    && bb(bhi[0]) + bb(r) > bb(c[0])
                    && bb(bhi[1]) + bb(r) > bb(c[1])
                    && bb(bhi[2]) + bb(r) > bb(c[2]);
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
