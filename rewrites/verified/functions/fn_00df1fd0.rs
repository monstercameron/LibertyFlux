// original: 0x00DF1FD0 UIMontageText::disable_text_controls

/// Disable the text controls owned by this montage view and store the
        /// returned HUD colour in a nested child control. `this` has nine child
/// pointers at offsets `0x1e4..=0x20c`; their vtables expose a no-argument
/// control hook at slot `0x1b0` and a one-argument update at slot `0x18`.
        /// The update receives zero. The first child also points to the target
        /// control at offset `0x1e4`. The HUD colour helper receives a pointer to a
/// stack copy of `this` and the colour index two; its returned pointer names
/// the colour word copied into child field `0x1e0`.
///
/// The class and child fields follow the 32-bit object layout, while the
/// child methods remain indirect so contract-provided vtables determine the
/// targets.
lf_checker_rt::export!(thiscall, rw_00df1fd0(this: u32) -> u32 {
    unsafe {
        const FIRST_CHILD: u32 = 0x1e4;
        const CHILD_OFFSETS: [u32; 9] = [
            0x1e4, 0x1e8, 0x1ec, 0x1f8, 0x1fc, 0x200, 0x204, 0x208, 0x20c,
        ];
        const CONTROL_HOOK_SLOT: u32 = 0x1b0;
        const UPDATE_SLOT: u32 = 0x18;
        const STORED_COLOUR: u32 = 0x1e0;
        const HUD_COLOUR_INDEX: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let first_child = read_u32(this.wrapping_add(FIRST_CHILD));
        let hook_table = read_u32(first_child);
        let hook_address = read_u32(hook_table.wrapping_add(CONTROL_HOOK_SLOT));
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(hook_address as usize);
        let _ = hook(first_child);

        for child_offset in CHILD_OFFSETS.iter().copied() {
            let child = read_u32(this.wrapping_add(child_offset));
            let table = read_u32(child);
            let update_address = read_u32(table.wrapping_add(UPDATE_SLOT));
            let update: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(update_address as usize);
            let _ = update(child, 0);
        }

        let mut this_copy = this;
        let colour_pointer = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            core::ptr::addr_of_mut!(this_copy) as u32,
            HUD_COLOUR_INDEX,
        );
        let colour = read_u32(colour_pointer);
        let colour_target = read_u32(first_child.wrapping_add(FIRST_CHILD));
        write_u32(colour_target.wrapping_add(STORED_COLOUR), colour);
        colour
    }
});
