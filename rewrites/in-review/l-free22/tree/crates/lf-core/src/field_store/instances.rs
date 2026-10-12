//! The field stores the verified rewrites were checked against: one entry per rewrite instance.
//!
//! Each entry gives the symbol name of a routine and its field store. The offsets, widths and
//! return kinds were read from the routines' own bodies; the test crate `lf-setdiff` runs every entry
//! against its verified rewrite, in this order, and the test fails on any difference.

use super::{FieldStore, FieldWidth, StoreResult};

/// One routine's field store, with its symbol name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedStore {
    /// The routine's symbol name.
    pub name: &'static str,
    /// What the routine does to its object.
    pub store: FieldStore,
}

/// The instances, in the order the test crate calls their rewrites.
pub const STORES: [NamedStore; 49] = [
    NamedStore {
        name: "UILayoutFrame::vf9",
        store: FieldStore {
            offset: 0xE8,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf10",
        store: FieldStore {
            offset: 0xE9,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf12",
        store: FieldStore {
            offset: 0xEA,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf77",
        store: FieldStore {
            offset: 0xC9,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf143",
        store: FieldStore {
            offset: 0x1F0,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf137",
        store: FieldStore {
            offset: 0x1EF,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutManager::vf124",
        store: FieldStore {
            offset: 0x1E0,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf132",
        store: FieldStore {
            offset: 0x1EC,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "NativeImpl_SET_CAMERA_AUTO_SCRIPT_ACTIVATION",
        store: FieldStore {
            offset: 0x534,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "input_store_byte_0xe6",
        store: FieldStore {
            offset: 0xE6,
            width: FieldWidth::Byte,
            result: StoreResult::Nothing,
        },
    },
    NamedStore {
        name: "audio_flag_store_17e0",
        store: FieldStore {
            offset: 0x17E0,
            width: FieldWidth::Byte,
            result: StoreResult::Nothing,
        },
    },
    NamedStore {
        name: "NativeImpl_ENABLE_GPS_IN_VEHICLE",
        store: FieldStore {
            offset: 0xB02,
            width: FieldWidth::Byte,
            result: StoreResult::Nothing,
        },
    },
    NamedStore {
        name: "audVehicleAudioEntity::set_loud_radio",
        store: FieldStore {
            offset: 0xB39,
            width: FieldWidth::Byte,
            result: StoreResult::Nothing,
        },
    },
    NamedStore {
        name: "ped_set_byte3a1",
        store: FieldStore {
            offset: 0x3A1,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "ped_set_byte3a2",
        store: FieldStore {
            offset: 0x3A2,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "CDummyPed::vf56",
        store: FieldStore {
            offset: 0x114,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf61",
        store: FieldStore {
            offset: 0xC8,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf70",
        store: FieldStore {
            offset: 0xC4,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf6",
        store: FieldStore {
            offset: 0xD1,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf79",
        store: FieldStore {
            offset: 0xC7,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf5",
        store: FieldStore {
            offset: 0xD0,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf17",
        store: FieldStore {
            offset: 0xEB,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf1",
        store: FieldStore {
            offset: 0xCA,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf98",
        store: FieldStore {
            offset: 0xE0,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf101",
        store: FieldStore {
            offset: 0xE4,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf92",
        store: FieldStore {
            offset: 0xD8,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf22",
        store: FieldStore {
            offset: 0xA0,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "UIFrame::vf115",
        store: FieldStore {
            offset: 0x1DC,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "uimouse_set_mode",
        store: FieldStore {
            offset: 0x1F8,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "NativeImpl_RETUNE_RADIO_TO_STATION_INDEX",
        store: FieldStore {
            offset: 0x80,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "text_set_field_9a4",
        store: FieldStore {
            offset: 0x9A4,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "CPlayerPed::vf78",
        store: FieldStore {
            offset: 0xEAC,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "CDummyPed::vf55",
        store: FieldStore {
            offset: 0x110,
            width: FieldWidth::Word,
            result: StoreResult::Value,
        },
    },
    NamedStore {
        name: "store_float_1b8",
        store: FieldStore {
            offset: 0x1B8,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "CPlayerPed::vf81",
        store: FieldStore {
            offset: 0xEE0,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "stream_state_set_float",
        store: FieldStore {
            offset: 0x10C,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "MenuOrHud::SetFloatField9B8",
        store: FieldStore {
            offset: 0x9B8,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf134",
        store: FieldStore {
            offset: 0x1F4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf128",
        store: FieldStore {
            offset: 0x1E8,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UIRowLayout::vf130",
        store: FieldStore {
            offset: 0x1E4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "field_e0_float_store",
        store: FieldStore {
            offset: 0xE0,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf95",
        store: FieldStore {
            offset: 0xDC,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "PedSetDecisionMakerIndex",
        store: FieldStore {
            offset: 0xD0,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "UILayoutFrame::vf89",
        store: FieldStore {
            offset: 0xD4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "field_e4_float_store",
        store: FieldStore {
            offset: 0xE4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "audGtaOcclusionGroup::vf2",
        store: FieldStore {
            offset: 0xA4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "audio_float_store_1750",
        store: FieldStore {
            offset: 0x1750,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    NamedStore {
        name: "input_store_float_0xc8",
        store: FieldStore {
            offset: 0xC8,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    NamedStore {
        name: "CPhysical::vf60",
        store: FieldStore {
            offset: 0x1F0,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
];
