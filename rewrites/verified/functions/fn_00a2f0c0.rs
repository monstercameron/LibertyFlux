// original: 0x00A2F0C0 rage::sysEmergencyAllocator::vf2

/// Reuses or grows the current thread's emergency allocation.
///
/// The stdcall entry takes a base size, an additional size, and an unsigned
/// allocation policy. It reads the policy-indexed thread record, obtains its
/// allocator buffer, and compares the buffer capacity with the wrapping sum
/// of the two requested sizes. Policies zero and two ask the thread allocator
/// for a replacement buffer with mode one or three, respectively. Policies
/// one and three grow through the general allocator with opposite argument
/// placement. Other policy values repeat the capacity query. The final
/// allocator method receives the base size, additional size, and effective
/// policy, and its return value is returned unchanged.
/// Helper calls are represented by scripted checker callees.
lf_checker_rt::export!(stdcall, rw_00a2f0c0(base_size: u32, additional_size: u32, policy: u32) -> u32 {
    const TLS_INDEX: u32 = 0x017A_BA14;
    const THREAD_RECORD_ALLOCATOR: usize = 4;
    const GENERAL_ALLOCATOR: u32 = 3;
    const GET_BUFFER: u32 = 1;
    const QUERY_CAPACITY: u32 = 2;
    const COMMIT_BUFFER: u32 = 4;

    unsafe {
        let slot = lf_checker_rt::global::<u32>(TLS_INDEX).read_unaligned() as usize;
        let thread_record = lf_checker_rt::tls_slot(slot) as *const u32;
        let allocator = thread_record.add(THREAD_RECORD_ALLOCATOR).read_unaligned();
        let requested_size = base_size.wrapping_add(additional_size);
        let mut effective_policy = policy;
        let mut buffer = lf_checker_rt::callee_thiscall!(
            GET_BUFFER,
            u32,
            allocator,
            policy
        );
        let mut capacity =
            lf_checker_rt::callee_thiscall!(QUERY_CAPACITY, u32, buffer);

        while capacity < requested_size {
            match policy {
                0 => {
                    effective_policy = 1;
                    buffer = lf_checker_rt::callee_thiscall!(
                        GET_BUFFER,
                        u32,
                        allocator,
                        effective_policy
                    );
                }
                1 => {
                    buffer = lf_checker_rt::callee_cdecl!(
                        GENERAL_ALLOCATOR,
                        u32,
                        base_size,
                        0u32
                    );
                }
                2 => {
                    effective_policy = 3;
                    buffer = lf_checker_rt::callee_thiscall!(
                        GET_BUFFER,
                        u32,
                        allocator,
                        effective_policy
                    );
                }
                3 => {
                    buffer = lf_checker_rt::callee_cdecl!(
                        GENERAL_ALLOCATOR,
                        u32,
                        0u32,
                        base_size
                    );
                }
                _ => {}
            }
            capacity = lf_checker_rt::callee_thiscall!(QUERY_CAPACITY, u32, buffer);
        }

        lf_checker_rt::callee_thiscall!(
            COMMIT_BUFFER,
            u32,
            buffer,
            base_size,
            additional_size,
            effective_policy
        )
    }
});
