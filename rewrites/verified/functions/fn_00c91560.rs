// original: 0x00c91560 fn_00c91560_copy_two_vectors

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

/// Copy two four-word vector records into the destination in the original
/// load/store order. Thiscall places the destination in ECX and takes the two
/// source pointers on the stack. EAX returns the last source word observed.
unsafe fn copy_vector_pair(destination: u32, first: u32, second: u32, write_last: bool) -> u32 {
    unsafe {
        let first_x = read_u32(first);
        let first_y = read_u32(first + 4);
        let first_z = read_u32(first + 8);
        write_u32(destination, first_x);
        write_u32(destination + 4, first_y);
        write_u32(destination + 8, first_z);
        let first_w = read_u32(first + 12);
        write_u32(destination + 12, first_w);

        let second_x = read_u32(second);
        let second_y = read_u32(second + 4);
        let second_z = read_u32(second + 8);
        write_u32(destination + 16, second_x);
        write_u32(destination + 20, second_y);
        write_u32(destination + 24, second_z);
        let second_w = read_u32(second + 12);
        if write_last {
            write_u32(destination + 28, second_w);
        }
        second_w
    }
}

lf_checker_rt::export!(thiscall, rw_00c91560_copy_two_vectors(destination: u32, first: u32, second: u32) -> u32 {
    unsafe { copy_vector_pair(destination, first, second, true) }
});
