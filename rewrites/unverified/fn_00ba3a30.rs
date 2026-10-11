// original: 0x00BA3A30 DecisionMaker_AddEventResponse

lf_checker_rt::export!(cdecl, rw_00ba3a30(
    decision_maker_handle: u32,
    response_kind: u32,
    response_value: u32,
    area_x: f32,
    area_y: f32,
    area_z: f32,
    area_range: f32,
    normalize_option: u32,
    trailing_option: u32,
) -> u32 {
    const HANDLE_KIND: u32 = 7;
    const MAX_EVENT_INDEX: u32 = 0x35;
    const ALLOCATION_BYTES: u32 = 0x20020;
    const DECISION_MAKER_GLOBAL: u32 = 0x0167E3B4;
    const LOOKUP_CALLEE: u32 = 1;
    const ALLOCATE_CALLEE: u32 = 2;
    const CONSTRUCT_CALLEE: u32 = 3;
    const ADD_RESPONSE_CALLEE: u32 = 4;
    const COOKIE_CHECK_CALLEE: u32 = 5;

    let event_index: u32 = lf_checker_rt::callee_cdecl!(
        LOOKUP_CALLEE,
        u32,
        decision_maker_handle,
        HANDLE_KIND,
    );
    if event_index > MAX_EVENT_INDEX {
        let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK_CALLEE, (), 0u32);
        return 0;
    }

    let decision_maker = unsafe {
        lf_checker_rt::global::<u32>(DECISION_MAKER_GLOBAL).read_volatile()
    };
    let decision_maker = if decision_maker == 0 {
        let allocated = lf_checker_rt::callee_cdecl!(
            ALLOCATE_CALLEE,
            u32,
            ALLOCATION_BYTES,
        );
        let created = if allocated == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CONSTRUCT_CALLEE, u32, allocated)
        };
        unsafe {
            lf_checker_rt::global::<u32>(DECISION_MAKER_GLOBAL)
                .write_volatile(created);
        }
        created
    } else {
        decision_maker
    };

    let records = [
        trailing_option,
        u32::from(normalize_option as u8 != 0),
        area_range.to_bits(),
        area_z.to_bits(),
        area_x.to_bits(),
        area_y.to_bits(),
    ];
    let later_record = unsafe { records.as_ptr().add(2) as u32 };
    let _ignored = lf_checker_rt::callee_thiscall!(
        ADD_RESPONSE_CALLEE,
        u32,
        decision_maker,
        event_index,
        response_kind,
        response_value,
        later_record,
        records.as_ptr() as u32,
        0u32,
    );
    let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK_CALLEE, (), 0u32);
    0
});
