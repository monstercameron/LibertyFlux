// original: 0x00ad7d80 rebind_ui_slot_row
use lf_checker_rt::{callee_cdecl, export, global};

//
// Snapshot three header words of the shared slot table into one indexed
// row, publish three tail pointers for that row and clear their targets,
// notify through a one-argument callback, then — unless a state flag says
// otherwise — restore the headers and clear the row again. A second state
// word selects a float-pair notifier on one value in eight. Returns the
// state flags on the early path, the masked selector otherwise, or the
// notifier's answer when it runs.
export!(cdecl, rw_00ad7d80(idx: i32) -> u32 {
    unsafe {
        let root = global::<u32>(0x012FB1B8).read();
        let flags = ((root.wrapping_add(0x8E8)) as *const u32).read();
        if flags & 0x10 == 0 && flags & 0x400 == 0 {
            return flags;
        }
        let stride = (idx as u32).wrapping_mul(4);
        let p = callee_cdecl!(1, u32,);
        let v0 = ((p.wrapping_add(0xE60)) as *const u32).read();
        let v1 = ((p.wrapping_add(0xE64)) as *const u32).read();
        let v2 = ((p.wrapping_add(0xE68)) as *const u32).read();
        (p.wrapping_add(stride).wrapping_add(0xE6C) as *mut u32).write(v0);
        let p = callee_cdecl!(1, u32,);
        let w = ((p.wrapping_add(0xE64)) as *const u32).read();
        (p.wrapping_add(stride).wrapping_add(0xECC) as *mut u32).write(w);
        let p = callee_cdecl!(1, u32,);
        let w = ((p.wrapping_add(0xE68)) as *const u32).read();
        (p.wrapping_add(stride).wrapping_add(0xF2C) as *mut u32).write(w);
        let p = callee_cdecl!(1, u32,);
        let tail = p.wrapping_add(stride).wrapping_add(0x104C);
        let mid = p.wrapping_add((idx.wrapping_add(0x3FB) as u32).wrapping_mul(4));
        let lo = p.wrapping_add((idx.wrapping_add(0x3E3) as u32).wrapping_mul(4));
        global::<u32>(0x01550DF0).write(tail);
        global::<u32>(0x01550DEC).write(mid);
        global::<u32>(0x01550DE8).write(lo);
        (lo as *mut u32).write(0);
        let mid_back = global::<u32>(0x01550DEC).read();
        (mid_back as *mut u32).write(0);
        let tail_back = global::<u32>(0x01550DF0).read();
        (tail_back as *mut u32).write(0);
        callee_cdecl!(2, u32, idx as u32);
        let root = global::<u32>(0x012FB1B8).read();
        let flag_byte = ((root.wrapping_add(0x8E8)) as *const u8).read();
        if flag_byte & 0x10 == 0 {
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(0xE60) as *mut u32).write(v0);
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(0xE64) as *mut u32).write(v1);
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(0xE68) as *mut u32).write(v2);
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(stride).wrapping_add(0xE6C) as *mut u32).write(0);
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(stride).wrapping_add(0xECC) as *mut u32).write(0);
            let p = callee_cdecl!(1, u32,);
            (p.wrapping_add(stride).wrapping_add(0xF2C) as *mut u32).write(0);
        }
        let sel = global::<u32>(0x01173604).read() & 7;
        if sel == 3 {
            let a = global::<f32>(0x0128E340).read();
            let b = global::<f32>(0x0128E344).read();
            callee_cdecl!(3, u32, a.to_bits(), b.to_bits(), 1)
        } else {
            sel
        }
    }
});
