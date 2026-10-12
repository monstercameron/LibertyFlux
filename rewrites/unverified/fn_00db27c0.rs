// original: 0x00DB27C0 UITexture::apply_hud_colour

/// Apply HUD colour 0x42 to a texture. The receiver uses vtable slots +0x140
/// for an early readiness query and +0x13C for its state setter. On the ready
/// path the byte result is returned immediately. Otherwise the method binds
/// its descriptor, optionally copies a nonempty name into +0x232, invokes the
/// state setter, stores the scripted colour word at +0x1E0 and writes the
/// supplied value and mode at +0x22C/+0x230. The fourth input is compared as
/// signed: only values greater than -1 are stored at +0x258.
lf_checker_rt::export!(thiscall, rw_00db27c0(
    this: u32,
    name: u32,
    descriptor: u32,
    color_value: u32,
    selected_index: u32,
) -> u32 {
    const VTABLE_READY_SLOT: u32 = 0x140;
    const VTABLE_SET_SLOT: u32 = 0x13C;
    const NAME_FIELD: u32 = 0x232;
    const NAME_FLAG: u32 = 0x257;
    const COLOR_FIELD: u32 = 0x1E0;
    const VALUE_FIELD: u32 = 0x22C;
    const MODE_FIELD: u32 = 0x230;
    const SELECTED_FIELD: u32 = 0x258;
    const NAME_CAPACITY: u32 = 0x25;

    unsafe {
        let vtable = (this as *const u32).read_unaligned();
        let ready_address = (vtable.wrapping_add(VTABLE_READY_SLOT) as *const u32).read_unaligned();
        let is_ready: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(ready_address as usize);
        let readiness = is_ready(this);
        if readiness != 0 { return readiness as u32; }

        let _ = lf_checker_rt::callee_thiscall!(
            2, u32, this, 0, 0, descriptor, u32::MAX
        );

        if name != 0 && (name as *const u8).read_unaligned() != 0 {
            let _ = lf_checker_rt::callee_cdecl!(
                3, u32, this.wrapping_add(NAME_FIELD), name, NAME_CAPACITY
            );
            (this.wrapping_add(NAME_FLAG) as *mut u8).write_unaligned(0);
        }

        let vtable = (this as *const u32).read_unaligned();
        let set_address = (vtable.wrapping_add(VTABLE_SET_SLOT) as *const u32).read_unaligned();
        let set_state: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(set_address as usize);
        let _ = set_state(this, 1);

        // The original passes a pointer to its incoming argument words. The
        // contract snapshots those three words while skipping only the address.
        let native_args = [descriptor, color_value, selected_index];
        let native_result = lf_checker_rt::callee_cdecl!(
            5, u32, native_args.as_ptr() as u32, 0x42
        );
        let color = (native_result as *const u32).read_unaligned();
        (this.wrapping_add(COLOR_FIELD) as *mut u32).write_unaligned(color);
        (this.wrapping_add(VALUE_FIELD) as *mut u32).write_unaligned(color_value);
        (this.wrapping_add(MODE_FIELD) as *mut u16).write_unaligned(1);
        if (selected_index as i32) > -1 {
            (this.wrapping_add(SELECTED_FIELD) as *mut u32).write_unaligned(selected_index);
        }
    }
    selected_index
});
