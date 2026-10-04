// original: 0x00979C10 audio_bus_param_store
/// Store one float parameter into a bus slot selected by two index bytes.
///
/// The destination row is `stride_count * index + table[lane]`, plus a fixed
/// field offset. An index byte of 0xFF stores through a null base and faults,
/// exactly like the original. Returns the table base. thiscall(obj, bits).
export!(thiscall, rw_s103_979c10(obj: *mut u8, bits: u32) -> u32 {
    unsafe {
        let idx = *obj.add(4);
        if idx == 0xFF {
            let zero: u32 = core::hint::black_box(0);
            core::ptr::write((zero.wrapping_add(0xCC) as usize) as *mut u32, bits);
            0
        } else {
            let lane = (*obj.add(0x40)) as u32;
            let count = *global::<u32>(0x115D968);
            let table = *global::<u32>(0x115D988);
            let slot = *((((table as usize).wrapping_add((lane.wrapping_mul(0x6F40)) as usize))
                .wrapping_add(0x6F14)) as *const u32);
            let dest = (count.wrapping_mul(idx as u32)).wrapping_add(slot).wrapping_add(0xCC);
            core::ptr::write((dest as usize) as *mut u32, bits);
            table
        }
    }
});
