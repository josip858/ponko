//! Explicit access to the dynamic ECS interface used by native bindings.
use crate::World;
use std::ops::{Deref, DerefMut};

/// Exclusive view of a world's dynamic component, resource, chunk and archetype APIs.
///
/// This borrows the same world; it does not copy storage or change the safety
/// requirements of dynamic methods. In particular, raw pointers remain subject
/// to the documented layout, lifetime and structural-mutation requirements.
/// Existing `World` methods remain callable for source compatibility.
pub struct RawWorld<'a>(&'a mut World);
impl Deref for RawWorld<'_> {
    type Target = World;
    fn deref(&self) -> &World {
        self.0
    }
}
impl DerefMut for RawWorld<'_> {
    fn deref_mut(&mut self) -> &mut World {
        self.0
    }
}

/// Shared dynamic view, for inspecting component metadata and raw pointers.
pub struct RawWorldRef<'a>(&'a World);
impl Deref for RawWorldRef<'_> {
    type Target = World;
    fn deref(&self) -> &World {
        self.0
    }
}
impl World {
    /// Borrows the dynamic/native interface without changing component identity.
    pub fn raw(&mut self) -> RawWorld<'_> {
        RawWorld(self)
    }
    /// Borrows read-only dynamic metadata and pointer access.
    pub fn raw_ref(&self) -> RawWorldRef<'_> {
        RawWorldRef(self)
    }
}

impl RawWorld<'_> {
    /// Pointer to a registered component, with the same validity rules as the native API.
    pub fn get_dynamic(&self, entity: crate::Entity, component: crate::ComponentId) -> Option<*mut u8> {
        self.0.get_dynamic(entity, component)
    }
    /// Removes one registered component; typed and dynamic access share storage.
    pub fn remove_dynamic(&mut self, entity: crate::Entity, component: crate::ComponentId) -> bool {
        self.0.remove_dynamic(entity, component)
    }
    /// Resolves the registered native or Rust component name.
    pub fn component_by_name(&self, name: &str) -> Option<crate::ComponentId> {
        self.0.component_by_name(name)
    }
    /// Iterates matching archetype chunks, preserving the world's borrow rules.
    pub fn for_each_chunk(&mut self, ids: &[crate::ComponentId], f: impl FnMut(crate::Chunk<'_>)) {
        self.0.for_each_chunk(ids, f);
    }
    /// Registered archetype tables for tooling.
    pub fn archetypes(&self) -> &[crate::Archetype] {
        self.0.archetypes()
    }
    /// Pointer to a native resource; replacement/removal invalidates the pointer.
    pub fn get_resource_dynamic(&self, name: &str) -> Option<*mut u8> {
        self.0.get_resource_dynamic(name)
    }
    /// Registered native resource layout.
    pub fn resource_layout_dynamic(&self, name: &str) -> Option<std::alloc::Layout> {
        self.0.resource_layout_dynamic(name)
    }
    /// Removes a native resource, invoking its existing disposal path.
    pub fn remove_resource_dynamic(&mut self, name: &str) -> bool {
        self.0.remove_resource_dynamic(name)
    }
}
impl RawWorldRef<'_> {
    /// Pointer to a registered component; the usual native pointer rules apply.
    pub fn get_dynamic(&self, entity: crate::Entity, component: crate::ComponentId) -> Option<*mut u8> {
        self.0.get_dynamic(entity, component)
    }
    /// Registered native resource pointer.
    pub fn get_resource_dynamic(&self, name: &str) -> Option<*mut u8> {
        self.0.get_resource_dynamic(name)
    }
    /// Registered native resource layout.
    pub fn resource_layout_dynamic(&self, name: &str) -> Option<std::alloc::Layout> {
        self.0.resource_layout_dynamic(name)
    }
    /// Looks up a registered native or typed component.
    pub fn component_by_name(&self, name: &str) -> Option<crate::ComponentId> {
        self.0.component_by_name(name)
    }
    /// Registered archetype tables.
    pub fn archetypes(&self) -> &[crate::Archetype] {
        self.0.archetypes()
    }
}
