// original: 0x00E3CF10 UIMontageMoviePlayer::set_movie_screen_visible

/// Compare the selected interface name with the image's MovieScreen name.
/// A mismatch returns one without changing state. A match toggles the
/// visibility byte, updates the child when present, then notifies this player.
/// The method is thiscall with one forwarded selector word. Its interface
/// getter is reached through the returned object's vtable at byte offset 0x48.
lf_checker_rt::export!(thiscall, rw_00e3cf10(this: u32, selector: u32) -> u32 {
    const CHILD_POINTER_OFFSET: u32 = 0x228;
    const VISIBILITY_BYTE_OFFSET: u32 = 0x238;
    const INTERFACE_VTABLE_OFFSET: u32 = 0x48;
    const GLOBAL_CONFIG_ADDRESS: u32 = 0x1158_1A4C;
    const IMAGE_BASE: u32 = 0x1000_0000;
    const IMAGE_NAME_RVA: u32 = 0x00B1_50B8;

    unsafe {
        let interface_object = lf_checker_rt::callee_thiscall!(
            1, u32, GLOBAL_CONFIG_ADDRESS, selector
        );
        let interface_vtable = (interface_object as *const u32).read_unaligned();
        let getter_address = (interface_vtable.wrapping_add(INTERFACE_VTABLE_OFFSET) as *const u32)
            .read_unaligned();
        let getter: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(getter_address as usize);
        let candidate_name = getter(interface_object);

        let reference_name = IMAGE_BASE.wrapping_add(IMAGE_NAME_RVA);
        let mut char_offset = 0u32;
        let same_name = loop {
            let candidate_byte = (candidate_name.wrapping_add(char_offset) as *const u8).read();
            let reference_byte = (reference_name.wrapping_add(char_offset) as *const u8).read();
            if candidate_byte != reference_byte { break false; }
            if candidate_byte == 0 { break true; }
            char_offset = char_offset.wrapping_add(1);
        };

        let child = (this.wrapping_add(CHILD_POINTER_OFFSET) as *const u32).read_unaligned();
        if !same_name { return 1; }

        let old_visible = (this.wrapping_add(VISIBILITY_BYTE_OFFSET) as *const u8).read() != 0;
        let new_visible = !old_visible;
        (this.wrapping_add(VISIBILITY_BYTE_OFFSET) as *mut u8).write(new_visible as u8);
        if child == 0 { return 0; }

        let _child_result = lf_checker_rt::callee_thiscall!(
            3, u32, child, new_visible as u32
        );
        lf_checker_rt::callee_thiscall!(4, u32, this, old_visible as u32)
    }
});
