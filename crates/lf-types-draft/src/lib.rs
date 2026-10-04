//! Draft 32-bit class layouts of the original game: structures and symbols, no code.
//!
//! # What this crate is
//!
//! Type and layout definitions (fields, offsets, sizes and stated base classes) for 340 classes
//! and records of the original 32-bit executable. They are a machine merge of the project's
//! class-layout analysis lanes (names starting `c-`), its native-handler analysis lanes (`n-`) and
//! the executable's class records. The crate holds no functions, no behaviour and no code from
//! the game; AGENTS.md allows structures and symbols in tracked files, and they are all it
//! contains. It is `no_std` and depends only on `lf-core`, for the layout assertions.
//!
//! # Status: every layout is Inferred
//!
//! No layout here is marked Verified: the merge records only static-analysis confidence, and
//! nothing in this crate records a comparison with the running game. Each type's doc gives the
//! merge's confidence in its size (the analysis lanes' own rating): high for 19 types, medium
//! for 65 and low for 256. Each field's doc gives the same rating for that field. The compile-time assertions prove that the Rust declarations
//! have the stated offsets and sizes on every target; they do not prove that the game uses them.
//!
//! Known limits of the merge (Verified by reading this crate):
//!
//! - Derived classes do not embed their bases. A derived type covers its base's bytes with its
//!   own fields and padding; the `Bases:` line names the base and its offset.
//! - A base type's layout includes fields first observed through derived classes (`via:` in the
//!   lane lists), so a base can be stated larger than a class derived from it. Of the 164
//!   base-and-derived pairs where both are in this crate, 83 have a base larger than the
//!   derived class. At least one size in each such pair is not the class's real size.
//! - A label's cited type (for example `s32` in `m_nLastDamageWeapon (s32, iv-sdk 1.0.7.0)`) can
//!   differ from the declared width. The declared width is what the lanes saw at that offset.
//!
//! # Conventions
//!
//! - Every type is `#[repr(C)]` and `Copy`, with fields in address order, so each field's offset
//!   follows from the widths before it.
//! - Pointer fields are [`Ptr32<T>`], always 4 bytes, so a layout is the same on a 64-bit host as
//!   in the 32-bit game. `Ptr32<()>` is a virtual-table pointer; `Ptr32<u8>` points at something
//!   whose type is not recovered.
//! - Bytes nobody has identified are padding: `_pad_XXXX: [u8; n]`, where `XXXX` is the hex
//!   offset of the first such byte, and `_pad_end` for the bytes from the last field to the
//!   asserted size. Their doc gives the range. Nothing may be read into them.
//! - Field names. Offset names give the hex offset and the kind the lanes saw: `field_18c`,
//!   `field_0x14`, `float_10`, `f32_58`, `ptr_c`, `bool_c`, `flag_0x87`, `u16_68`, `f_14_bool`,
//!   `f_f4_u32_or_pointer`. The spellings differ by lane. These stay until evidence names the field;
//!   a guessed name is worse than an honest offset. Descriptive names are the lanes' labels;
//!   some labels cite IV-SDK 1.0.7.0 names, which AGENTS.md allows reading for names. A label
//!   that repeats inside one type gets `_2`, `_3` and so on.
//! - Field docs read `label (confidence: c, kind: k, lanes: ...)`. `label` is the lanes' own
//!   name; `via:X` means the field was observed through derived class `X`, and "moved from
//!   siblings" lists the derived classes it was merged up from.
//! - Type docs give the original name, `Size: 0xNN (confidence)`, `Bases: Base@offset` and the
//!   lanes that contributed.
//! - Type names are the original name in CamelCase with the namespace prefix folded in and
//!   template punctuation dropped: `rage::phBound` is [`RagePhBound`], `CDrawPhoneDC_NY` is
//!   [`CDrawPhoneDCNY`].
//! - Assertions: every type has `lf_core::assert_size!` with the merged size rounded up to 4
//!   (the comment beside it records the merged size), and every non-padding field has
//!   `lf_core::assert_offset!`: 340 size and 5629 offset assertions for 5629 fields and
//!   2185 padding runs. A layout that drifts on any target, including `i686-pc-windows-msvc`,
//!   fails the build.
//!
//! # Modules
//!
//! Types are grouped by subsystem, judged from the original name and, for draw commands, the
//! stated base class. Every module is re-exported at the crate root, so `lf_types_draft::CPed`
//! and `lf_types_draft::peds::CPed` are the same type and paths from before the split still work.
//!
//! | Module | Types | Holds |
//! |---|---|---|
//! | [`animation`] | 22 | RAGE animation and motion-tree classes. |
//! | [`audio`] | 17 | Audio entities, sounds, voices, effects and stream readers. |
//! | [`base`] | 10 | Foundation types: RAGE base classes, pools, maths and system services. |
//! | [`cameras`] | 5 | Cameras and viewports. |
//! | [`draw_commands`] | 44 | Draw commands: every class whose stated base is `CBaseDC`. |
//! | [`entities`] | 18 | World entities, model information and pickups. |
//! | [`events`] | 11 | Events: the `CEvent` hierarchy. |
//! | [`input`] | 3 | Controller input. |
//! | [`natural_motion`] | 29 | NaturalMotion behaviour engine (`ART::` and `NMutils::` namespaces). |
//! | [`network`] | 11 | Network session tasks, leaderboards and network blenders. |
//! | [`peds`] | 16 | Peds and players. |
//! | [`physics`] | 40 | RAGE physics, fragments and collision detection. |
//! | [`render`] | 20 | Rendering: render phases, particles, sky and shader fragments. |
//! | [`script`] | 3 | Script threads. |
//! | [`streaming`] | 4 | Resource paging and loading. |
//! | [`tasks`] | 41 | Tasks: the `CTask` hierarchy and task records. |
//! | [`ui`] | 37 | Front-end user interface, HTML rendering, replay editor and blip records. |
//! | [`unclassified`] | 2 | Types whose subsystem their names do not show. |
//! | [`vehicles`] | 7 | Vehicles. |
//!
//! ```
//! use core::mem::size_of;
//! use lf_types_draft::{entities, CPhysical, Ptr32};
//!
//! // The root path and the module path name one type.
//! fn same(layout: entities::CPhysical) -> CPhysical {
//!     layout
//! }
//! let _ = same;
//! assert_eq!(size_of::<CPhysical>(), 0xea8);
//! assert_eq!(size_of::<Ptr32<CPhysical>>(), 4);
//! ```
//!
//! # Field renames
//!
//! Fields are renamed only on evidence in this repository. The 42 renames below all have the
//! same evidence: the field was named by flattening a Hungarian-notation label in its own doc
//! comment, sometimes with the label's cited type and source run into the name. The new name is
//! that label without `m_` and the one-letter type prefix, in snake case. Two labels in `CPhysical`
//! reduce to the same name, so each keeps the type its label cites. The doc comment, offset and
//! type of every renamed field are unchanged.
//!
//! Not renamed, because another field of the same type already has the reduced name and two
//! claims would share one name:
//!
//! - `CPlayerInfo::m_nnevertired` (would become `never_tired`)
//! - `CPlayerInfo::m_nmaxhealth` (would become `max_health`)
//! - `CPlayerInfo::m_pplayerped` (would become `player_ped`)
//!
//! | Type | Old name | New name | Evidence (the field's own label) |
//! |---|---|---|---|
//! | `CBaseModelInfo` | `m_parchetype_pharchetypegta_iv_sdk_1_0_7_0` | `archetype` | `m_pArchetype (phArchetypeGta*, iv-sdk 1.0.7.0)` |
//! | `CBaseModelInfo` | `m_pdrawablestruct_tdrawablestruct_iv_sdk_1_0_7_0` | `drawable_struct` | `m_pDrawableStruct (tDrawableStruct*, iv-sdk 1.0.7.0)` |
//! | `CBaseModelInfo` | `m_nideflags_u32_iv_sdk_1_0_7_0` | `ide_flags` | `m_nIDEFlags (u32, iv-sdk 1.0.7.0)` |
//! | `CBaseModelInfo` | `m_ntexdictionary_s16_iv_sdk_1_0_7_0` | `tex_dictionary` | `m_nTexDictionary (s16, iv-sdk 1.0.7.0)` |
//! | `CBaseModelInfo` | `m_nanimindex_s8_iv_sdk_1_0_7_0` | `anim_index` | `m_nAnimIndex (s8, iv-sdk 1.0.7.0)` |
//! | `CEntity` | `m_ncurrentweaponslot` | `current_weapon_slot` | `m_nCurrentWeaponSlot` |
//! | `CEntity` | `m_placement_csimpletransform_iv_sdk_1_0_7_0` | `placement` | `m_placement (CSimpleTransform, iv-sdk 1.0.7.0)` |
//! | `CEntity` | `m_nentityflags2_u32_iv_sdk_1_0_7_0` | `entity_flags2` | `m_nEntityFlags2 (u32, iv-sdk 1.0.7.0)` |
//! | `CEntity` | `m_pdrawableptr_tobjectdrawable_iv_sdk_1_0_7_0` | `drawable_ptr` | `m_pDrawablePtr (tObjectDrawable*, iv-sdk 1.0.7.0)` |
//! | `CEntity` | `m_hinterior` | `interior` | `m_hInterior` |
//! | `CEntity` | `m_fdrawdistance_f32_iv_sdk_1_0_7_0` | `draw_distance` | `m_fDrawDistance (f32, iv-sdk 1.0.7.0)` |
//! | `CNetBlenderDummyPed` | `m_bdirty` | `dirty` | `m_bDirty?` |
//! | `CNetBlenderDummyPed` | `m_pentity` | `entity` | `m_pEntity` |
//! | `CNetBlenderDummyPed` | `m_fheading_0` | `heading_0` | `m_fHeading[0]` |
//! | `CNetBlenderDummyPed` | `m_hblend_0` | `blend_0` | `m_hBlend[0]` |
//! | `CNetBlenderDummyPed` | `m_fheading_1` | `heading_1` | `m_fHeading[1]` |
//! | `CNetBlenderDummyPed` | `m_hblend_1` | `blend_1` | `m_hBlend[1]` |
//! | `CNetBlenderDummyPed` | `m_fheading_2` | `heading_2` | `m_fHeading[2]` |
//! | `CNetBlenderDummyPed` | `m_hblend_2` | `blend_2` | `m_hBlend[2]` |
//! | `CNetBlenderDummyPed` | `m_binitialised` | `initialised` | `m_bInitialised` |
//! | `CPed` | `m_fpedhealth` | `ped_health` | `m_fPedHealth` |
//! | `CPed` | `m_nplayerindex_u8_iv_sdk_1_0_7_0` | `player_index` | `m_nPlayerIndex (u8, iv-sdk 1.0.7.0)` |
//! | `CPed` | `m_bisplayer_u8_iv_sdk_1_0_7_0` | `is_player` | `m_bIsPlayer (u8, iv-sdk 1.0.7.0)` |
//! | `CPedModelInfo` | `m_ngestureanimindex_s32_iv_sdk_1_0_7_0` | `gesture_anim_index` | `m_nGestureAnimIndex (s32, iv-sdk 1.0.7.0)` |
//! | `CPedModelInfo` | `m_nfacialanimindex_s32_iv_sdk_1_0_7_0` | `facial_anim_index` | `m_nFacialAnimIndex (s32, iv-sdk 1.0.7.0)` |
//! | `CPedModelInfo` | `m_bstreamedped` | `streamed_ped` | `m_bStreamedPed` |
//! | `CPedModelInfo` | `m_npedtype` | `ped_type` | `m_nPedType` |
//! | `CPhysical` | `m_fpercentsubmerged` | `percent_submerged` | `m_fPercentSubmerged` |
//! | `CPhysical` | `m_vattachoffset_cvector_iv_sdk_1_0_7_0` | `attach_offset_vector` | `m_vAttachOffset (CVector, iv-sdk 1.0.7.0)` |
//! | `CPhysical` | `m_qattachoffset_cquaternion_iv_sdk_1_0_7_0` | `attach_offset_quaternion` | `m_qAttachOffset (CQuaternion, iv-sdk 1.0.7.0)` |
//! | `CPhysical` | `m_plastdamageentity` | `last_damage_entity` | `m_pLastDamageEntity` |
//! | `CPhysical` | `m_nlastdamageweapon_s32_iv_sdk_1_0_7_0` | `last_damage_weapon` | `m_nLastDamageWeapon (s32, iv-sdk 1.0.7.0)` |
//! | `CPhysical` | `m_fhealth_f32_iv_sdk_1_0_7_0` | `health` | `m_fHealth (f32, iv-sdk 1.0.7.0)` |
//! | `CPhysical` | `m_pentityignoredcollision` | `entity_ignored_collision` | `m_pEntityIgnoredCollision` |
//! | `CPlayerInfo` | `m_pplayerped2` | `player_ped2` | `m_pPlayerPed2` |
//! | `CPlayerInfo` | `m_fstamina` | `stamina` | `m_fStamina` |
//! | `CPlayerInfo` | `m_nlasthitpedtime` | `last_hit_ped_time` | `m_nLastHitPedTime` |
//! | `CPlayerInfo` | `m_ncontrolflags` | `control_flags` | `m_nControlFlags` |
//! | `CPlayerInfo` | `m_nplayerid` | `player_id` | `m_nPlayerId` |
//! | `CPlayerInfo` | `m_nstate` | `state` | `m_nState` |
//! | `CPlayerInfo` | `m_nmaxarmor` | `max_armor` | `m_nMaxArmor` |
//! | `CPlayerInfo` | `m_ponlyenterthisvehicle` | `only_enter_this_vehicle` | `m_pOnlyEnterThisVehicle` |
#![no_std]
// Field and type docs repeat the analysis lanes' labels verbatim (`field_4`, `u32_or_ptr`,
// `via:CPed`); putting each in backticks would rewrite the recorded evidence.
#![allow(clippy::doc_markdown)]
// Padding runs are public `_pad_XXXX` fields so that every type can be built field by field; the
// leading underscore marks bytes nobody has identified (see "Conventions" above).
#![allow(clippy::pub_underscore_fields)]

use core::marker::PhantomData;

pub mod animation;
pub mod audio;
pub mod base;
pub mod cameras;
pub mod draw_commands;
pub mod entities;
pub mod events;
pub mod input;
pub mod natural_motion;
pub mod network;
pub mod peds;
pub mod physics;
pub mod render;
pub mod script;
pub mod streaming;
pub mod tasks;
pub mod ui;
pub mod unclassified;
pub mod vehicles;

pub use animation::*;
pub use audio::*;
pub use base::*;
pub use cameras::*;
pub use draw_commands::*;
pub use entities::*;
pub use events::*;
pub use input::*;
pub use natural_motion::*;
pub use network::*;
pub use peds::*;
pub use physics::*;
pub use render::*;
pub use script::*;
pub use streaming::*;
pub use tasks::*;
pub use ui::*;
pub use unclassified::*;
pub use vehicles::*;

/// A 32-bit game pointer: the address only, 4 bytes on every host target.
///
/// `T` names the pointee for readers; nothing here dereferences it. Field 0 is the address as the
/// game stores it; field 1 is a zero-sized marker.
#[repr(transparent)]
pub struct Ptr32<T = ()>(pub u32, pub PhantomData<*const T>);

impl<T> Copy for Ptr32<T> {}

impl<T> Clone for Ptr32<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> core::fmt::Debug for Ptr32<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Ptr32(0x{:08x})", self.0)
    }
}

lf_core::assert_size!(Ptr32<()>, 4);
lf_core::assert_size!(Ptr32<u8>, 4);
