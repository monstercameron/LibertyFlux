// original: 0x00B229D0 ide_short_record_accumulate
/// Store two signed-sized counters and snapshot their previous accumulated
/// totals into a 20-byte short-record header. The totals at 0x01657618 and
/// 0x0165761c then advance by the supplied unsigned 32-bit values.
/// Thiscall, two stack arguments, no declared return value.
lf_checker_rt::export!(thiscall, rw_00b229d0(this: u32, amount_a: u32, amount_b: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        const TOTAL_A: u32 = 0x0165_7618;
        const TOTAL_B: u32 = 0x0165_761C;
        write_u32(this, amount_a);
        write_u32(this.wrapping_add(4), amount_b);
        let total_a = lf_checker_rt::relocated(TOTAL_A) as u32;
        let total_b = lf_checker_rt::relocated(TOTAL_B) as u32;
        write_u32(this.wrapping_add(8), read_u32(total_a));
        write_u32(this.wrapping_add(0x10), read_u32(total_b));
        write_u32(total_a, read_u32(total_a).wrapping_add(amount_a));
        write_u32(total_b, read_u32(total_b).wrapping_add(amount_b));
        0
    }
});
