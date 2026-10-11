// original: 0x00DAA170 task_dispatch_detail

/// Reproduce the descriptor-free rejection path of task dispatch.
///
/// When the optional descriptor is zero, the routine submits a default query.
/// A false answer causes one fallback query; a true answer reaches a local
/// status check that still rejects the request. Both tested outcomes return
/// zero without writing caller outputs. The contract cycles the query answer
/// between false and true. Descriptor-backed work and the ready-status
/// continuation, including the later double-vector call, are outside this
/// proof.
fn run_task_dispatch_detail(
    context: u32,
    _target_vector: u32,
    _output_scalar: u32,
    sample_vector: u32,
    _output_id: u32,
    _output_point: u32,
    _output_axis: u32,
    _output_normal: u32,
    _maximum_distance: u32,
    _fallback_vector: u32,
    descriptor: u32,
    omit_fallback_query: bool,
) -> u32 {
    unsafe {
        const QUERY_LIMIT_BITS: u32 = 0x3f4c_cccd;
        const QUERY_TOLERANCE_BITS: u32 = 0x3a83_126f;
        const FALLBACK_TOLERANCE_BITS: u32 = 0x3dcc_cccd;
        const QUERY_KIND: u32 = 0x8e;

        if descriptor != 0 {
            return 0;
        }

        let mut query_scratch = [0u32; 4];
        let mut query_state = [0u32; 4];
        let first_answer: u32 = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            context,
            sample_vector,
            QUERY_LIMIT_BITS,
            query_scratch.as_mut_ptr() as u32,
            query_state.as_mut_ptr() as u32,
            QUERY_TOLERANCE_BITS,
            1u32,
            QUERY_KIND,
            0u32,
        );

        if (first_answer as u8) != 0 {
            return 0;
        }
        if omit_fallback_query {
            return 0;
        }

        let fallback_answer: u32 = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            context,
            sample_vector,
            QUERY_LIMIT_BITS,
            query_scratch.as_mut_ptr() as u32,
            query_state.as_mut_ptr() as u32,
            FALLBACK_TOLERANCE_BITS,
            0u32,
            QUERY_KIND,
            0u32,
        );

        if (fallback_answer as u8) != 0 {
            return 0;
        }
        0
    }
}

lf_checker_rt::export!(
    cdecl,
    rw_00DAA170(
        context: u32,
        target_vector: u32,
        output_scalar: u32,
        sample_vector: u32,
        output_id: u32,
        output_point: u32,
        output_axis: u32,
        output_normal: u32,
        maximum_distance: u32,
        fallback_vector: u32,
        descriptor: u32,
    ) -> u32 {
        run_task_dispatch_detail(
            context,
            target_vector,
            output_scalar,
            sample_vector,
            output_id,
            output_point,
            output_axis,
            output_normal,
            maximum_distance,
            fallback_vector,
            descriptor,
            false,
        )
    }
);

lf_checker_rt::export!(
    cdecl,
    mut_00DAA170(
        context: u32,
        target_vector: u32,
        output_scalar: u32,
        sample_vector: u32,
        output_id: u32,
        output_point: u32,
        output_axis: u32,
        output_normal: u32,
        maximum_distance: u32,
        fallback_vector: u32,
        descriptor: u32,
    ) -> u32 {
        run_task_dispatch_detail(
            context,
            target_vector,
            output_scalar,
            sample_vector,
            output_id,
            output_point,
            output_axis,
            output_normal,
            maximum_distance,
            fallback_vector,
            descriptor,
            true,
        )
    }
);
