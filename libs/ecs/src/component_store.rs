use crate::{unsafe_option_vec::UnsafeOptionVec, Component, Entity};
use hashbrown::HashMap;
use safety_guard::safety;
use std::{any::TypeId, cell::UnsafeCell, num::NonZeroUsize};

/// A container for components.
#[derive(Debug)]
pub struct ComponentStore {
    components: UnsafeCell<HashMap<TypeId, UnsafeOptionVec>>,
    next_entity: usize,
}

impl ComponentStore {
    /// Creates a new, empty ComponentStore.
    pub fn new() -> ComponentStore {
        ComponentStore {
            components: UnsafeCell::new(HashMap::new()),
            next_entity: 1,
        }
    }

    /// Returns an iterator over all entities.
    pub fn iter_entities(&self) -> impl Clone + Iterator<Item = Entity> {
        (1..self.next_entity).map(|n| {
            NonZeroUsize::new(n)
                .map(Entity)
                .expect("impossible case? entity 0")
        })
    }

    /// Creates a new entity.
    pub fn new_entity(&mut self) -> Entity {
        let n = self.next_entity;
        self.next_entity = self
            .next_entity
            .checked_add(1)
            .expect("too many entities allocated");
        NonZeroUsize::new(n)
            .map(Entity)
            .expect("impossible case? entity 0")
    }

    /// Gets a component for a given entity.
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        unsafe {
            self.components
                .get()
                .as_ref()
                .unwrap()
                .get(&TypeId::of::<T>())
                .and_then(|vec| vec.get::<T>(entity.0.get()))
        }
    }

    /// Gets a component for a given entity.
    pub fn get_mut_component<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        unsafe { self.unsafe_get_mut_component(entity) }
    }

    /// Sets a component for a given entity.
    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        unsafe {
            self.components
                .get()
                .as_mut()
                .unwrap()
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set(entity.0.get(), Some(component))
        }
    }

    /// Removes a component from a given entity.
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        unsafe {
            self.components
                .get()
                .as_mut()
                .unwrap()
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set::<T>(entity.0.get(), None)
        }
    }

    /// Gets a component for a given entity. This is unsafe since it makes it possible to have two
    /// mutable references to the same component if called twice with the same T.
    #[safety(
        "The references returned by calling this function with the same T must not exist at once."
    )]
    pub unsafe fn unsafe_get_mut_component<T: Component>(&self, entity: Entity) -> Option<&mut T> {
        self.components
            .get()
            .as_mut()
            .unwrap()
            .get_mut(&TypeId::of::<T>())
            .and_then(|vec| vec.get_mut::<T>(entity.0.get()))
    }
}

unsafe impl Send for ComponentStore {}
unsafe impl Sync for ComponentStore {}
