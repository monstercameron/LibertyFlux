//! Collaborator boundary for the per-item montage visibility commit.

use lf_core::Handle32;

/// Opaque registry service used to resolve the item key.
pub struct RegistryTag;
/// Opaque shared listener that receives the committed index.
pub struct ListenerTag;
/// Opaque context for the item notification helper.
pub struct NotifyContextTag;
/// Opaque receiver for the final forwarded call.
pub struct TailContextTag;

/// Which side supplies a geometry value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryObject {
    /// The resolved item source.
    Source,
    /// The montage container receiving the commit.
    Container,
}

/// Which of the four rectangle values a collaborator returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryValue {
    /// Horizontal half-extent.
    HalfWidth,
    /// Horizontal center.
    CenterX,
    /// Vertical half-extent.
    HalfHeight,
    /// Vertical center.
    CenterY,
}

/// Named game string roles used by this method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemTag {
    /// Identity compared against the resolved source tag.
    SourceIdentity,
    /// String role passed to the notification helper.
    Notification,
    /// String role passed to configuration construction.
    FloatConfiguration,
}

/// Viewport values read by the original from shared globals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Signed viewport width before scaling.
    pub width: i32,
    /// Signed viewport height before scaling.
    pub height: i32,
    /// Bit pattern of the width scale.
    pub width_scale_bits: u32,
    /// Bit pattern of the height scale.
    pub height_scale_bits: u32,
}

/// Shared values used at the boundary of the item commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemGlobals {
    /// Registry service handle used by both source lookups.
    pub registry: Handle32<RegistryTag>,
    /// Shared listener object used by failure and success paths.
    pub listener: Handle32<ListenerTag>,
    /// Receiver for the optional item notification.
    pub notify_context: Handle32<NotifyContextTag>,
    /// Receiver used by the final forwarded call.
    pub tail_context: Handle32<TailContextTag>,
    /// Listener word that selects its primary failure helper.
    pub listener_word_204: u32,
    /// Listener word that selects its secondary failure helper.
    pub listener_word_200: u32,
    /// Viewport dimensions and scale bit patterns.
    pub viewport: Viewport,
}

/// Calls made by `commit_item` that are not lifted yet.
pub trait ItemWorld {
    /// Reads one rectangle value from the source or container.
    fn geometry(&mut self, object: GeometryObject, value: GeometryValue) -> f32;
    /// Resolves an item key through the registry.
    fn lookup_source(
        &mut self,
        registry: Handle32<RegistryTag>,
        key: u32,
    ) -> Handle32<super::world::SrcTag>;
    /// Runs the list's low-byte gatekeeper.
    fn gatekeeper(&mut self, list: Handle32<super::world::ListTag>) -> u32;
    /// Reads the resolved source's identity tag.
    fn source_tag(&mut self, source: Handle32<super::world::SrcTag>) -> u32;
    /// Resolves one of the named item string roles.
    fn tag_id(&mut self, tag: ItemTag) -> u32;
    /// Reads the source's low-byte notification guard.
    fn source_subflag(&mut self, source: Handle32<super::world::SrcTag>) -> u32;
    /// Sends the optional item notification.
    fn notify(&mut self, context: Handle32<NotifyContextTag>, tag: ItemTag);
    /// Calls the primary listener helper on a failed item gate.
    fn listener_primary(&mut self, listener: Handle32<ListenerTag>);
    /// Calls the secondary listener helper on a failed item gate.
    fn listener_secondary(&mut self, listener: Handle32<ListenerTag>);
    /// Calls the container's failure entry.
    fn own_fail(&mut self, own: Handle32<super::world::OwnTag>);
    /// Calls the container's dismiss entry.
    fn own_dismiss(&mut self, own: Handle32<super::world::OwnTag>);
    /// Reads the resolved source's index.
    fn source_index(&mut self, source: Handle32<super::world::SrcTag>) -> u32;
    /// Resolves a list sink value for the given key.
    fn list_sink(&mut self, list: Handle32<super::world::ListTag>, key: u32) -> u32;
    /// Posts a nonnegative list index with its mode argument.
    fn post_index(&mut self, list: Handle32<super::world::ListTag>, index: u32, mode: u32);
    /// Sends the selected index to the configuration collaborator.
    fn tail_notify(&mut self, cfg: Handle32<super::world::CfgTag>, value: u32);
    /// Writes the selected index to the listener's result word.
    fn listener_result(&mut self, listener: Handle32<ListenerTag>, value: u32);
    /// Reads the list's inner-object link.
    fn list_inner(&mut self, list: Handle32<super::world::ListTag>) -> Handle32<super::world::InnerTag>;
    /// Reads the inner object's completion value.
    fn inner_done(&mut self, inner: Handle32<super::world::InnerTag>) -> u32;
    /// Computes the float value used to build configuration.
    fn measure(&mut self, source: Handle32<super::world::SrcTag>, value: u32) -> f32;
    /// Builds a configuration object from a named tag and float bits.
    fn build_configuration(
        &mut self,
        registry: Handle32<RegistryTag>,
        tag: ItemTag,
        value_bits: u32,
    ) -> u32;
    /// Releases a temporary configuration object.
    fn release_configuration(&mut self, configuration: u32);
    /// Forwards the final inner-object completion value.
    fn tail_forward(&mut self, context: Handle32<TailContextTag>, value: u32);
}
