// original: 0x009273B0 ui_draw_textured_rect (proposed)
use lf_checker_rt::{callee_thiscall, export, global};

/// Submit one textured screen rectangle to the UI renderer.
///
/// Does nothing unless the global UI-enabled byte is set. `target` and
/// `source` are objects whose first word points to a table of thiscall
/// getters: offset 0x20 answers an integer width and offset 0x24 an integer
/// height. The destination corners `(x0, y0)` and `(x1, y1)` in pixels are
/// mapped to -1..1 by dividing by the target's size, doubling and
/// subtracting one; the source corners `(u0, v0)` and `(u1, v1)` are divided
/// by the source's size, and half a source texel (half the reciprocal size)
/// is added to each. The function then registers the source object and
/// the raw word `tag` with the renderer through two calls on the first
/// renderer global, and finally calls the draw routine of the second
/// renderer global with eleven words: the four destination values (the two
/// vertical ones negated), `depth` passed through as raw float bits, the
/// four texel-adjusted source values, -1 and the global blend word.
///
/// Returns nothing.
export!(cdecl, rw_9273b0(
    target: u32, source: u32, x0: i32, y0: i32, x1: i32, y1: i32, depth: u32,
    u0: i32, v0: i32, u1: i32, v1: i32, tag: u32
) -> () {
    unsafe {
        const ENABLED: u32 = 0x01036780; // byte
        const RENDERER_REGISTER: u32 = 0x0119CFF8;
        const RENDERER_DRAW: u32 = 0x0119D000;
        const HANDLE_SOURCE: u32 = 0x0119D074;
        const HANDLE_TAG: u32 = 0x0119D070;
        const BLEND_WORD: u32 = 0x0119D080;
        const SLOT_WIDTH: u32 = 0x20;
        const SLOT_HEIGHT: u32 = 0x24;
        const NO_CLIP: u32 = 0xFFFF_FFFF;

        if global::<u8>(ENABLED).read() == 0 {
            return;
        }

        // Call the argument-less thiscall getter at table offset `slot` of
        // `obj` and convert its integer answer to a float.
        let getter = |obj: u32, slot: u32| -> f32 {
            let table = (obj as *const u32).read();
            let entry = (table.wrapping_add(slot) as *const u32).read();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(entry as usize);
            (f(obj) as i32) as f32
        };
        // Pixel coordinate to -1..1 over an extent.
        let ndc = |pixel: i32, extent: f32| -> f32 { ((pixel as f32) / extent) * 2.0 - 1.0 };
        // Pixel coordinate to 0..1 over an extent.
        let unit = |pixel: i32, extent: f32| -> f32 { (pixel as f32) / extent };

        // Destination, in the original's call order: width, width, height, height.
        let dx0 = ndc(x0, getter(target, SLOT_WIDTH));
        let dx1 = ndc(x1, getter(target, SLOT_WIDTH));
        let dy0 = ndc(y0, getter(target, SLOT_HEIGHT));
        let dy1 = ndc(y1, getter(target, SLOT_HEIGHT));

        // Reciprocal source extents.
        let inv_w = 1.0f32 / getter(source, SLOT_WIDTH);
        let inv_h = 1.0f32 / getter(source, SLOT_HEIGHT);

        callee_thiscall!(3, u32, global::<u32>(RENDERER_REGISTER).read(),
            global::<u32>(HANDLE_SOURCE).read(), source);
        callee_thiscall!(4, u32, global::<u32>(RENDERER_REGISTER).read(),
            global::<u32>(HANDLE_TAG).read(), tag);

        let su0 = unit(u0, getter(source, SLOT_WIDTH));
        let su1 = unit(u1, getter(source, SLOT_WIDTH));
        let sv0 = unit(v0, getter(source, SLOT_HEIGHT));
        let sv1 = unit(v1, getter(source, SLOT_HEIGHT));

        // Half a source texel.
        let half_w = inv_w * 0.5;
        let half_h = inv_h * 0.5;

        callee_thiscall!(5, u32, global::<u32>(RENDERER_DRAW).read(),
            dx0.to_bits(),
            (-dy0).to_bits(),
            dx1.to_bits(),
            (-dy1).to_bits(),
            depth,
            (half_w + su0).to_bits(),
            (half_h + sv0).to_bits(),
            (half_w + su1).to_bits(),
            (half_h + sv1).to_bits(),
            NO_CLIP,
            global::<u32>(BLEND_WORD).read());
    }
});
