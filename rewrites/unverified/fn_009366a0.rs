// original: 0x009366A0 submit_network_transition_values

/// Submit five stack values to the transition helper when the global gate is
/// nonzero. A nonzero receiver field at `+0xd70` enables a two-stage object
/// table lookup: the table index comes from `+0xd6c`, and an entry whose byte
/// at `+8` is zero falls back to the global current index. The selected object
/// must have a nonzero dword at `+0x4c`. The helper receives the receiver's
/// fields at `+0xd60` and `+0xd74`, the receiver pointer, and the five input
/// words in their original order. Four of the input words are forwarded as
/// raw float bits without arithmetic. This is a thiscall routine with five
/// stack arguments and no defined return value.
lf_checker_rt::export!(thiscall, rw_009366a0(
    this: u32,
    first_float_bits: u32,
    second_float_bits: u32,
    mode: u32,
    fourth_float_bits: u32,
    fifth_float_bits: u32
) -> () {
    unsafe {
        const RECEIVER_GATE: u32 = 0xd70;
        const TABLE_INDEX: u32 = 0xd6c;
        const HELPER_OBJECT_A: u32 = 0xd60;
        const HELPER_OBJECT_B: u32 = 0xd74;
        const TABLE_GLOBAL: u32 = 0x0118_f6f8;
        const CURRENT_INDEX_GLOBAL: u32 = 0x0103_4494;
        const GLOBAL_GATE: u32 = 0x0116_0c94;

        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        if unsafe { lf_checker_rt::global::<u32>(GLOBAL_GATE).read_unaligned() } == 0 {
            return;
        }

        if read_u32(this.wrapping_add(RECEIVER_GATE)) != 0 {
            let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL) } as u32;
            let primary_index = read_u32(this.wrapping_add(TABLE_INDEX));
            let mut selected = read_u32(
                table.wrapping_add(primary_index.wrapping_mul(4))
            );
            if selected == 0 {
                return;
            }
            if read_u8(selected.wrapping_add(8)) == 0 {
                let fallback_index = unsafe {
                    lf_checker_rt::global::<u32>(CURRENT_INDEX_GLOBAL).read_unaligned()
                };
                selected = read_u32(table.wrapping_add(fallback_index.wrapping_mul(4)));
            }
            if read_u32(selected.wrapping_add(0x4c)) == 0 {
                return;
            }
        }

        let helper_object_a = read_u32(this.wrapping_add(HELPER_OBJECT_A));
        let helper_object_b = read_u32(this.wrapping_add(HELPER_OBJECT_B));
        let _ = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            helper_object_a,
            this,
            helper_object_b,
            first_float_bits,
            second_float_bits,
            mode,
            fourth_float_bits,
            fifth_float_bits
        );
    }
});
