// original: 0x00ad8960 emit_ui_quad_batch
use lf_checker_rt::{callee_cdecl, export, global};

//
// Emit one UI quad batch by one of three paths. When the immediate flag is
// set, begin a six-vertex batch and push six tinted vertices through the
// vertex sink, then tail into the flush routine; when both flags are clear,
// begin a four-vertex batch and push four of the same vertices, then tail
// into the same flush. When only the buffered flag is set, stamp six
// vertices directly into the shared vertex buffer, advance its count by six
// and return the new count (the flush-on-full jump past the buffer end is
// not taken for the small counts this contract uses). The integer arguments
// are biased corner coordinates, the floats are depths. Returns the flush
// answer on the immediate paths and the new buffer count on the buffered
// path.
export!(cdecl, rw_00ad8960(
    a0: u32, a1: i32, a2: i32, a3: i32, a4: f32, a5: f32, a6: f32, a7: f32,
) -> u32 {
    unsafe {
        let g0 = global::<u32>(0x015932C0).read() as i32;
        let g1 = global::<u32>(0x0158D5FC).read() as i32;
        let a2s = a2.wrapping_sub(g0);
        let a3s = a3.wrapping_sub(g0);
        let esi = (a0 as i32).wrapping_sub(g1);
        let edi = a1.wrapping_sub(g1);
        let esif = esi as f32;
        let edif = edi as f32;
        let a2f = a2s as f32;
        let a3f = a3s as f32;
        if global::<u8>(0x0154E2B6).read() != 0 {
            callee_cdecl!(1, u32, 2, 6);
            sink_vert(esif, a3f, a6);
            sink_vert(esif, a2f, a4);
            sink_vert(edif, a2f, a5);
            sink_vert(esif, a3f, a6);
            sink_vert(edif, a3f, a7);
            sink_vert(edif, a2f, a5);
            return callee_cdecl!(
                3, u32, a0, a1 as u32, a2s as u32, a3s as u32, a4.to_bits(),
                a5.to_bits(), a6.to_bits(), a7.to_bits()
            );
        }
        if global::<u8>(0x0154E2B5).read() == 0 {
            callee_cdecl!(1, u32, 4, 4);
            sink_vert(esif, a2f, a4);
            sink_vert(edif, a2f, a5);
            sink_vert(esif, a3f, a6);
            sink_vert(edif, a3f, a7);
            return callee_cdecl!(
                3, u32, a0, a1 as u32, a2s as u32, a3s as u32, a4.to_bits(),
                a5.to_bits(), a6.to_bits(), a7.to_bits()
            );
        }
        let p = global::<u32>(0x01550EA4).read();
        let d = if ((p.wrapping_add(6)) as *const u8).read() != 0 {
            ((p.wrapping_add(8)) as *const u32).read()
        } else {
            0
        };
        let n = global::<u32>(0x01550EA8).read();
        let base = d.wrapping_add(n.wrapping_mul(36));
        stamp_vert(base, 0x00, esif, a2f, a4);
        stamp_vert(base, 0x24, esif, a2f, a4);
        stamp_vert(base, 0x48, edif, a2f, a5);
        stamp_vert(base, 0x6C, esif, a3f, a6);
        stamp_vert(base, 0x90, edif, a3f, a7);
        stamp_vert(base, 0xB4, edif, a3f, a7);
        let n6 = n.wrapping_add(6);
        global::<u32>(0x01550EA8).write(n6);
        n6
    }
});

/// One vertex through the intercepted sink: position, two zero words, a
/// 1.0 homogeneous coordinate and the opaque tint word.
#[inline(always)]
fn sink_vert(x: f32, y: f32, z: f32) {
    callee_cdecl!(
        2, u32, x.to_bits(), y.to_bits(), z.to_bits(), 0, 0, 0x3F800000,
        0xFF000000
    );
}

/// One nine-word vertex stamped into the shared buffer at base+off.
#[inline(always)]
unsafe fn stamp_vert(base: u32, off: u32, x: f32, y: f32, z: f32) {
    unsafe {
        let v = base.wrapping_add(off);
        ((v.wrapping_add(0x00)) as *mut u32).write(x.to_bits());
        ((v.wrapping_add(0x04)) as *mut u32).write(y.to_bits());
        ((v.wrapping_add(0x08)) as *mut u32).write(z.to_bits());
        ((v.wrapping_add(0x0C)) as *mut u32).write(0);
        ((v.wrapping_add(0x10)) as *mut u32).write(0);
        ((v.wrapping_add(0x14)) as *mut u32).write(0x3F800000);
        ((v.wrapping_add(0x18)) as *mut u32).write(0xFF000000);
        ((v.wrapping_add(0x1C)) as *mut u32).write(0);
        ((v.wrapping_add(0x20)) as *mut u32).write(0);
    }
}
