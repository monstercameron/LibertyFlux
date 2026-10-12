// original: 0x00DB2B50 UITexture::snapshot_active_slot_for_render

/// Snapshot the current texture state into the per-slot render record selected
/// by the image global at 0x017A65A8. Each record is 0x44 bytes, beginning
/// at receiver offset 0xF0 plus 0x44 times the unsigned global index. The method
/// copies the receiver's integer, 64-bit bit-pattern and byte fields, stores the
/// virtual readiness result as a byte, clears the receiver's active-slot byte,
/// and marks the selected record valid. The virtual getter is called after the
/// copied record fields are written.
lf_checker_rt::export!(thiscall, rw_00db2b50(this: u32) -> u32 {
    const GLOBAL_SLOT_FILE_VA: u32 = 0x017A65A8;
    const VTABLE_READY_SLOT: u32 = 0x124;
    const RECORD_BASE: u32 = 0xF0;
    const RECORD_STRIDE: u32 = 0x44;

    unsafe {
        let slot_index = lf_checker_rt::global::<u32>(GLOBAL_SLOT_FILE_VA).read_unaligned();
        let record = this.wrapping_add(RECORD_BASE.wrapping_add(slot_index.wrapping_mul(RECORD_STRIDE)));

        let source_1e4 = (this.wrapping_add(0x1E4) as *const u32).read_unaligned();
        let source_1e8 = (this.wrapping_add(0x1E8) as *const u32).read_unaligned();
        let source_1ec = (this.wrapping_add(0x1EC) as *const u32).read_unaligned();
        let source_1f0 = (this.wrapping_add(0x1F0) as *const u32).read_unaligned();
        (record as *mut u32).write_unaligned(source_1e4);
        (record.wrapping_add(4) as *mut u32).write_unaligned(source_1e8);
        (record.wrapping_add(8) as *mut u32).write_unaligned(source_1ec);
        (record.wrapping_add(12) as *mut u32).write_unaligned(source_1f0);

        let source_1f4 = (this.wrapping_add(0x1F4) as *const u64).read_unaligned();
        let source_1fc = (this.wrapping_add(0x1FC) as *const u64).read_unaligned();
        (record.wrapping_add(0x20) as *mut u64).write_unaligned(source_1f4);
        (record.wrapping_add(0x28) as *mut u64).write_unaligned(source_1fc);

        let source_1e0 = (this.wrapping_add(0x1E0) as *const u32).read_unaligned();
        let source_224 = (this.wrapping_add(0x224) as *const u32).read_unaligned();
        let normalized_active_byte = (this.wrapping_add(0x22B) as *const u8).read_unaligned();
        (record.wrapping_add(0x30) as *mut u32).write_unaligned(source_1e0);
        (record.wrapping_add(0x34) as *mut u32).write_unaligned(source_224);
        (record.wrapping_add(0x38) as *mut u32).write_unaligned(u32::from(normalized_active_byte));

        let copied_color_a = (this.wrapping_add(0x229) as *const u8).read_unaligned();
        let copied_color_b = (this.wrapping_add(0x22A) as *const u8).read_unaligned();
        let copied_color_c = (this.wrapping_add(0x228) as *const u8).read_unaligned();
        (record.wrapping_add(0x3C) as *mut u8).write_unaligned(copied_color_a);
        (record.wrapping_add(0x3D) as *mut u8).write_unaligned(copied_color_b);
        (record.wrapping_add(0x3E) as *mut u8).write_unaligned(copied_color_c);

        let vtable = (this as *const u32).read_unaligned();
        let getter_address = (vtable.wrapping_add(VTABLE_READY_SLOT) as *const u32).read_unaligned();
        let get_ready: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(getter_address as usize);
        let ready_byte = get_ready(this);
        (record.wrapping_add(0x3F) as *mut u8).write_unaligned(ready_byte);
        (this.wrapping_add(0x22B) as *mut u8).write_unaligned(0);
        (record.wrapping_add(0x40) as *mut u8).write_unaligned(1);
    }
    0
});
