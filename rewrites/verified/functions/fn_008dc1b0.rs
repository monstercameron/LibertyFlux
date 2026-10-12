// original: 0x008DC1B0 CSetupDefLight::CSetupDefLight

/// Build a definition-light command from four input blocks and three float bit patterns. The
/// first block supplies four words, the second and third supply four words each, and the fourth
/// supplies the final four words. The function copies the fields in the original order, writes
/// the scalar identifiers at +0x50 and +0x60, advances the serial token, and returns `this`.
/// All float inputs are transported as their original 32-bit bit patterns.
lf_checker_rt::export!(thiscall, rw_008dc1b0(this: u32, definition_id: u32, block_a: u32, float_a_bits: u32, block_b: u32, block_c: u32, float_b_bits: u32, float_c_bits: u32, block_d: u32, final_word: u32) -> u32 {
    const COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81974;
    const TOKEN_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;
    unsafe {
        let object = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(COUNTER_VA);
        let previous_token = core::ptr::read_volatile(object.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_volatile(object.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = previous_token ^ ((previous_token ^ serial) & TOKEN_MASK);
        core::ptr::write_volatile(object.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));

        let definition_id = definition_id;
        core::ptr::write_volatile(object.wrapping_add(0x50).cast::<u32>(), definition_id);
        core::ptr::write_volatile(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));

        let block_a = block_a as *const u8;
        let a_y = core::ptr::read_volatile(block_a.wrapping_add(4).cast::<u32>());
        let a_z = core::ptr::read_volatile(block_a.wrapping_add(8).cast::<u32>());
        let a_x = core::ptr::read_volatile(block_a.cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x14).cast::<u32>(), a_y);
        core::ptr::write_volatile(object.wrapping_add(0x10).cast::<u32>(), a_x);
        core::ptr::write_volatile(object.wrapping_add(0x18).cast::<u32>(), a_z);
        let a_w = core::ptr::read_volatile(block_a.wrapping_add(0x0c).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x1c).cast::<u32>(), a_w);
        core::ptr::write_volatile(object.wrapping_add(0x54).cast::<u32>(), float_a_bits);

        let block_b = block_b as *const u8;
        let b_y = core::ptr::read_volatile(block_b.wrapping_add(4).cast::<u32>());
        let b_z = core::ptr::read_volatile(block_b.wrapping_add(8).cast::<u32>());
        let b_x = core::ptr::read_volatile(block_b.cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x20).cast::<u32>(), b_x);
        core::ptr::write_volatile(object.wrapping_add(0x24).cast::<u32>(), b_y);
        core::ptr::write_volatile(object.wrapping_add(0x28).cast::<u32>(), b_z);
        let b_w = core::ptr::read_volatile(block_b.wrapping_add(0x0c).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x2c).cast::<u32>(), b_w);

        let block_c = block_c as *const u8;
        let c_y = core::ptr::read_volatile(block_c.wrapping_add(4).cast::<u32>());
        let c_x = core::ptr::read_volatile(block_c.cast::<u32>());
        let c_z = core::ptr::read_volatile(block_c.wrapping_add(8).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x30).cast::<u32>(), c_x);
        core::ptr::write_volatile(object.wrapping_add(0x34).cast::<u32>(), c_y);
        core::ptr::write_volatile(object.wrapping_add(0x38).cast::<u32>(), c_z);
        core::ptr::write_volatile(object.wrapping_add(0x58).cast::<u32>(), float_b_bits);
        let c_w = core::ptr::read_volatile(block_c.wrapping_add(0x0c).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x3c).cast::<u32>(), c_w);
        core::ptr::write_volatile(object.wrapping_add(0x5c).cast::<u32>(), float_c_bits);

        let block_d = block_d as *const u8;
        let d_x = core::ptr::read_volatile(block_d.cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x40).cast::<u32>(), d_x);
        let d_y = core::ptr::read_volatile(block_d.wrapping_add(4).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x44).cast::<u32>(), d_y);
        let d_z = core::ptr::read_volatile(block_d.wrapping_add(8).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x48).cast::<u32>(), d_z);
        let d_w = core::ptr::read_volatile(block_d.wrapping_add(0x0c).cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(0x4c).cast::<u32>(), d_w);
        core::ptr::write_volatile(object.wrapping_add(0x60).cast::<u32>(), final_word);
    }
    this
});
