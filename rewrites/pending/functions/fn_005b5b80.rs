// original: 0x005B5B80 record_table_init
/// Initialises the three-slot record table at the shared buffer: stamps the
/// (1,0), (3,0) and (3,5) tag pairs with cleared payload words and sets the
/// slot count to 3. Returns the buffer address with its low word replaced by
/// the final count, as the original leaves it in EAX.
export!(cdecl, rw_005B5B80() -> u32 {
    unsafe {
        let base = *global::<u32>(0x019D2F18);
        let w = base as *mut u32;
        w.add(0).write(1);
        w.add(1).write(0);
        w.add(2).write(0xFFFF_FFFF);
        w.add(3).write(0xFFFF_FFFF);
        w.add(4).write(3);
        w.add(5).write(0);
        w.add(6).write(0xFFFF_FFFF);
        w.add(7).write(0xFFFF_FFFF);
        w.add(8).write(3);
        w.add(9).write(5);
        w.add(10).write(0xFFFF_FFFF);
        w.add(11).write(0xFFFF_FFFF);
        *global::<u16>(0x019D2F1C) = 3;
        (base & 0xFFFF_0000) | 3
    }
});
