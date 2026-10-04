// original: 0x00b56e30 forward_scaled_level
// rs05f0: scale a per-object level and forward it to two timing callbacks.
//
// `tag` is passed through opaquely; `obj` selects the work. When `obj` is
// null the function does nothing. Otherwise it multiplies the level stored
// at `obj+0xC` by `scale` and passes the product, together with the owner's
// field at `this+0x1A40`, through an apply step (thiscall/4) and a notify
// step (thiscall/2, receiving the tag back).
export!(thiscall, rw_b56e30(this: *const u8, tag: u32, obj: *const u8, scale: f32) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        let level = *(obj.add(0x0C) as *const f32);
        let field = *(this.add(0x1A40) as *const u32);
        let apply: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        apply(obj as u32, (level * scale).to_bits(), field, 0, 1);
        let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        notify(field, tag, 0);
    }
});
