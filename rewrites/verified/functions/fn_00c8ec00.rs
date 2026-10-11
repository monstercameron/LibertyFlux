// original: 0x00c8ec00 fn_00c8ec00_copy_motion_slots

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Copy two four-word motion records from the object into separate caller
/// buffers without changing any payload bits. The object is in ECX, the two
/// output pointers are stack arguments, and the original pops both arguments.
/// It returns the last word copied. The checker contract uses disjoint buffers;
/// aliasing between either output and the source object remains unproved.
const MOTION_RECORD_A: u32 = 0x250;
const MOTION_RECORD_B: u32 = 0x260;

unsafe fn copy_motion_slots(object: u32, first_output: u32, second_output: u32, write_final_word: bool) -> u32 {
    unsafe {
        let first_y = read_u32(object + MOTION_RECORD_A + 4);
        let first_z = read_u32(object + MOTION_RECORD_A + 8);
        let first_x = read_u32(object + MOTION_RECORD_A);
        write_u32(first_output, first_x);
        write_u32(first_output + 4, first_y);
        write_u32(first_output + 8, first_z);
        let first_w = read_u32(object + MOTION_RECORD_A + 12);
        write_u32(first_output + 12, first_w);

        let second_x = read_u32(object + MOTION_RECORD_B);
        let second_y = read_u32(object + MOTION_RECORD_B + 4);
        let second_z = read_u32(object + MOTION_RECORD_B + 8);
        write_u32(second_output, second_x);
        write_u32(second_output + 4, second_y);
        write_u32(second_output + 8, second_z);
        let second_w = read_u32(object + MOTION_RECORD_B + 12);
        if write_final_word {
            write_u32(second_output + 12, second_w);
        }
        second_w
    }
}

lf_checker_rt::export!(thiscall, rw_00c8ec00_copy_motion_slots(object: u32, first_output: u32, second_output: u32) -> u32 {
    unsafe { copy_motion_slots(object, first_output, second_output, true) }
});
