//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `lf_core::field_store::instances::STORES`. The result is
//! `None` for a rewrite that returns nothing.

/// The rewrite of `UILayoutFrame::vf9`, called with the object at `base` and the value bits `value`.
pub fn call_0(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db1940::rw_00db1940(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf10`, called with the object at `base` and the value bits `value`.
pub fn call_1(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db1950::rw_00db1950(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf12`, called with the object at `base` and the value bits `value`.
pub fn call_2(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db1960::rw_00db1960(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf77`, called with the object at `base` and the value bits `value`.
pub fn call_3(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db1970::rw_00db1970(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf143`, called with the object at `base` and the value bits `value`.
pub fn call_4(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de4590::rw_00de4590(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf137`, called with the object at `base` and the value bits `value`.
pub fn call_5(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de45a0::rw_00de45a0(base as usize as u32, value))
}

/// The rewrite of `UILayoutManager::vf124`, called with the object at `base` and the value bits `value`.
pub fn call_6(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de45c0::rw_00de45c0(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf132`, called with the object at `base` and the value bits `value`.
pub fn call_7(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de46a0::rw_00de46a0(base as usize as u32, value))
}

/// The rewrite of `NativeImpl_SET_CAMERA_AUTO_SCRIPT_ACTIVATION`, called with the object at `base` and the value bits `value`.
pub fn call_8(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_009b80f0::rw_009b80f0(base as usize as u32, value))
}

/// The rewrite of `input_store_byte_0xe6`, called with the object at `base` and the value bits `value`.
pub fn call_9(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_008981a0::rw_008981a0(base as usize as u32, value);
    None
}

/// The rewrite of `audio_flag_store_17e0`, called with the object at `base` and the value bits `value`.
pub fn call_10(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_008b0f10::rw_008b0f10(base as usize as u32, value);
    None
}

/// The rewrite of `NativeImpl_ENABLE_GPS_IN_VEHICLE`, called with the object at `base` and the value bits `value`.
pub fn call_11(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_0098ee70::rw_nativeimpl_enable_gps_in_vehicle(base as usize as u32, value);
    None
}

/// The rewrite of `audVehicleAudioEntity::set_loud_radio`, called with the object at `base` and the value bits `value`.
pub fn call_12(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_0098eec0::rw_aud_vehicle_set_loud_radio(base as usize as u32, value);
    None
}

/// The rewrite of `ped_set_byte3a1`, called with the object at `base` and the value bits `value`.
pub fn call_13(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00b4f300::rw_00b4f300(base as usize as u32, value))
}

/// The rewrite of `ped_set_byte3a2`, called with the object at `base` and the value bits `value`.
pub fn call_14(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00b4fea0::rw_00b4fea0(base as usize as u32, value))
}

/// The rewrite of `CDummyPed::vf56`, called with the object at `base` and the value bits `value`.
pub fn call_15(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00b4ffb0::rw_00b4ffb0(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf61`, called with the object at `base` and the value bits `value`.
pub fn call_16(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f4f0::rw_00c0f4f0(base, value))
}

/// The rewrite of `UILayoutFrame::vf70`, called with the object at `base` and the value bits `value`.
pub fn call_17(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f500::rw_00c0f500(base, value))
}

/// The rewrite of `UILayoutFrame::vf6`, called with the object at `base` and the value bits `value`.
pub fn call_18(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f530::rw_00c0f530(base, value))
}

/// The rewrite of `UILayoutFrame::vf79`, called with the object at `base` and the value bits `value`.
pub fn call_19(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f5a0::rw_00c0f5a0(base, value))
}

/// The rewrite of `UILayoutFrame::vf5`, called with the object at `base` and the value bits `value`.
pub fn call_20(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f5b0::rw_00c0f5b0(base, value))
}

/// The rewrite of `UILayoutFrame::vf17`, called with the object at `base` and the value bits `value`.
pub fn call_21(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f840::rw_00c0f840(base, value))
}

/// The rewrite of `UILayoutFrame::vf1`, called with the object at `base` and the value bits `value`.
pub fn call_22(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f8d0::rw_00c0f8d0(base, value))
}

/// The rewrite of `UILayoutFrame::vf98`, called with the object at `base` and the value bits `value`.
pub fn call_23(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f830::rw_00c0f830(base, value))
}

/// The rewrite of `UILayoutFrame::vf101`, called with the object at `base` and the value bits `value`.
pub fn call_24(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f850::rw_00c0f850(base, value))
}

/// The rewrite of `UILayoutFrame::vf92`, called with the object at `base` and the value bits `value`.
pub fn call_25(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f860::rw_00c0f860(base, value))
}

/// The rewrite of `UILayoutFrame::vf22`, called with the object at `base` and the value bits `value`.
pub fn call_26(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f8c0::rw_00c0f8c0(base, value))
}

/// The rewrite of `UIFrame::vf115`, called with the object at `base` and the value bits `value`.
pub fn call_27(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c0f8e0::rw_00c0f8e0(base, value))
}

/// The rewrite of `uimouse_set_mode`, called with the object at `base` and the value bits `value`.
pub fn call_28(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db4e00::rw_00db4e00(base as usize as u32, value))
}

/// The rewrite of `NativeImpl_RETUNE_RADIO_TO_STATION_INDEX`, called with the object at `base` and the value bits `value`.
pub fn call_29(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_009a4700::rw_009a4700(base as usize as u32, value))
}

/// The rewrite of `text_set_field_9a4`, called with the object at `base` and the value bits `value`.
pub fn call_30(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_008fc0d0::rw_008fc0d0(base as usize as u32, value))
}

/// The rewrite of `CPlayerPed::vf78`, called with the object at `base` and the value bits `value`.
pub fn call_31(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a2d420::rw_00a2d420(base as usize as u32, value))
}

/// The rewrite of `CDummyPed::vf55`, called with the object at `base` and the value bits `value`.
pub fn call_32(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00b4ffc0::rw_00b4ffc0(base as usize as u32, value))
}

/// The rewrite of `store_float_1b8`, called with the object at `base` and the value bits `value`.
pub fn call_33(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00b06490::rw_00b06490(base as usize as u32, value))
}

/// The rewrite of `CPlayerPed::vf81`, called with the object at `base` and the value bits `value`.
pub fn call_34(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a2d390::rw_00a2d390(base as usize as u32, value))
}

/// The rewrite of `stream_state_set_float`, called with the object at `base` and the value bits `value`.
pub fn call_35(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00c08c60::rw_00c08c60(base as usize as u32, value))
}

/// The rewrite of `MenuOrHud::SetFloatField9B8`, called with the object at `base` and the value bits `value`.
pub fn call_36(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_008fbdd0::rw_008fbdd0(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf134`, called with the object at `base` and the value bits `value`.
pub fn call_37(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00db7350::rw_00db7350(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf128`, called with the object at `base` and the value bits `value`.
pub fn call_38(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de45b0::rw_00de45b0(base as usize as u32, value))
}

/// The rewrite of `UIRowLayout::vf130`, called with the object at `base` and the value bits `value`.
pub fn call_39(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00de46b0::rw_00de46b0(base as usize as u32, value))
}

/// The rewrite of `field_e0_float_store`, called with the object at `base` and the value bits `value`.
pub fn call_40(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a64c00::rw_00a64c00(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf95`, called with the object at `base` and the value bits `value`.
pub fn call_41(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a64d30::rw_00a64d30(base as usize as u32, value))
}

/// The rewrite of `PedSetDecisionMakerIndex`, called with the object at `base` and the value bits `value`.
pub fn call_42(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a64d40::rw_00a64d40(base as usize as u32, value))
}

/// The rewrite of `UILayoutFrame::vf89`, called with the object at `base` and the value bits `value`.
pub fn call_43(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a64d50::rw_00a64d50(base as usize as u32, value))
}

/// The rewrite of `field_e4_float_store`, called with the object at `base` and the value bits `value`.
pub fn call_44(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_00a64d60::rw_00a64d60(base as usize as u32, value))
}

/// The rewrite of `audGtaOcclusionGroup::vf2`, called with the object at `base` and the value bits `value`.
pub fn call_45(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_009748b0::rw_009748b0(base as usize as u32, value))
}

/// The rewrite of `audio_float_store_1750`, called with the object at `base` and the value bits `value`.
pub fn call_46(base: *mut u8, value: u32) -> Option<u32> {
    Some(crate::rewrites::m_008b0fe0::rw_008B0FE0(base as usize as u32, value))
}

/// The rewrite of `input_store_float_0xc8`, called with the object at `base` and the value bits `value`.
pub fn call_47(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_00897f90::rw_00897f90(base as usize as u32, value);
    None
}

/// The rewrite of `CPhysical::vf60`, called with the object at `base` and the value bits `value`.
pub fn call_48(base: *mut u8, value: u32) -> Option<u32> {
    crate::rewrites::m_00b25230::rw_b25230(base, f32::from_bits(value));
    None
}

/// The call table, in the order of the instances table.
pub const CALLS: [fn(*mut u8, u32) -> Option<u32>; 49] = [call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8, call_9, call_10, call_11, call_12, call_13, call_14, call_15, call_16, call_17, call_18, call_19, call_20, call_21, call_22, call_23, call_24, call_25, call_26, call_27, call_28, call_29, call_30, call_31, call_32, call_33, call_34, call_35, call_36, call_37, call_38, call_39, call_40, call_41, call_42, call_43, call_44, call_45, call_46, call_47, call_48];
