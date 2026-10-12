// original: 0x008DC2C0 CUnLockRenderTargetDC

/// The function initializes a render-target unlock command with fixed default color and flag values. A null optional descriptor leaves those defaults; a non-null descriptor replaces the packed color words and sets the present flag. It advances the serial token and returns the command pointer.
lf_checker_rt::export!(thiscall, rw_008dc2c0(this: u32, resource_id: u32, source: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E819C8;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;

    unsafe {
        let command = this as *mut u8;
        let serial_counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial_for_token = core::ptr::read_volatile(serial_counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial_for_token) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(serial_counter.cast_const());
        core::ptr::write_volatile(serial_counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        core::ptr::write_unaligned(command.wrapping_add(0x0c).cast::<u32>(), 0x3f80_0000);
        core::ptr::write_unaligned(command.wrapping_add(0x10).cast::<u32>(), 0x3f80_0000);
        core::ptr::write_unaligned(command.wrapping_add(0x14).cast::<u32>(), 0xff00_0000);
        core::ptr::write_unaligned(command.wrapping_add(0x18).cast::<u32>(), 0);
        core::ptr::write_unaligned(command.wrapping_add(0x1c).cast::<u32>(), 0);
        core::ptr::write_unaligned(command.wrapping_add(0x20).cast::<u32>(), 0x0100_0000);
        core::ptr::write(command.wrapping_add(0x24), 1);
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), resource_id);
        core::ptr::write(command.wrapping_add(0x28), 0);
        if source != 0 {
            let source = source as *const u8;
            let block0 = core::ptr::read_unaligned(source.cast::<u64>());
            let block1 = core::ptr::read_unaligned(source.wrapping_add(8).cast::<u64>());
            let block2 = core::ptr::read_unaligned(source.wrapping_add(0x10).cast::<u64>());
            let block3 = core::ptr::read_unaligned(source.wrapping_add(0x18).cast::<u32>());
            core::ptr::write_unaligned(command.wrapping_add(0x0c).cast::<u64>(), block0);
            core::ptr::write_unaligned(command.wrapping_add(0x14).cast::<u64>(), block1);
            core::ptr::write_unaligned(command.wrapping_add(0x1c).cast::<u64>(), block2);
            core::ptr::write_unaligned(command.wrapping_add(0x24).cast::<u32>(), if false { block3.wrapping_add(1) } else { block3 });
            core::ptr::write(command.wrapping_add(0x28), 1);
        }
    }
    this
});
