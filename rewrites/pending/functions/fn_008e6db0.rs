// original: 0x008e6db0 append_global_record
/// Append a 32-byte record (four words from `src`, two extra words) to the
/// global array while its count is below 64, then bump the count.
export!(stdcall, rw_008e6db0(src: *const u8, extra0: u32, extra1: u32) -> () {
    unsafe {
        let count = read_unaligned(global::<u32>(0x01176E48));
        if count < 0x40 {
            let dst = global::<u8>(0x01176E80).add(count as usize * 32);
            write_unaligned(dst as *mut u32, read_unaligned(src as *const u32));
            write_unaligned(
                dst.add(4) as *mut u32,
                read_unaligned(src.add(4) as *const u32),
            );
            write_unaligned(
                dst.add(8) as *mut u32,
                read_unaligned(src.add(8) as *const u32),
            );
            write_unaligned(
                dst.add(12) as *mut u32,
                read_unaligned(src.add(12) as *const u32),
            );
            write_unaligned(dst.add(16) as *mut u32, extra0);
            write_unaligned(dst.add(20) as *mut u32, extra1);
            write_unaligned(global::<u32>(0x01176E48), count + 1);
        }
    }
});
