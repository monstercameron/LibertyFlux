// original: 0x00B13310 object_pack_audio_record

/// Pack the audio vector through the fixed byte converter, then copy the
/// record's remaining words and derive its compact flag byte.
///
/// `this` is the destination record and `source` points to the source
/// structure. Its field at `+0x20` is a pointer to the packed float data.
/// The helper receives that stored pointer as its
/// single stack argument. The remaining source fields at `+0x24`, `+0x28`,
/// `+0x210`, `+0x214` and `+0x22a` are copied to destination offsets
/// `+0x18`, `+0x1c`, `+0x20`, `+0x24` and `+0x28`. Bits `0x40`, `0x80`,
/// `0x100`, `0x200` and `0x1000` of the source flags map to destination
/// flag bits `0x04`, `0x08`, `0x10`, `0x20` and `0x40`. The helper's return
/// value is forwarded. This is a thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00B13310(this: u32, source: u32) -> u32 {
    unsafe {
        const SOURCE_FLOATS: u32 = 0x20;
        const SOURCE_FLAGS: u32 = 0x118;
        const DEST_FLAGS: u32 = 0x29;
        let float_data = (source.wrapping_add(SOURCE_FLOATS) as *const u32).read_unaligned();
        let converted = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            float_data,
        );
        let read = |offset: u32| (source.wrapping_add(offset) as *const u32).read_unaligned();
        let write = |offset: u32, value: u32| {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        };
        write(0x18, read(0x24));
        write(0x1c, read(0x28));
        write(0x20, read(0x210));
        write(0x24, read(0x214));
        (this.wrapping_add(0x28) as *mut u8).write((source.wrapping_add(0x22a) as *const u8).read());

        let source_flags = read(SOURCE_FLAGS);
        let mut packed_flags = 0u8;
        if source_flags & 0x40 != 0 { packed_flags |= 0x04; }
        if source_flags & 0x80 != 0 { packed_flags |= 0x08; }
        if source_flags & 0x100 != 0 { packed_flags |= 0x10; }
        if source_flags & 0x200 != 0 { packed_flags |= 0x20; }
        if source_flags & 0x1000 != 0 { packed_flags |= 0x40; }
        (this.wrapping_add(DEST_FLAGS) as *mut u8).write(packed_flags);
        converted
    }
});
