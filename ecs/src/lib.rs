pub trait Component: 'static {}

// =================================================================== //
// ============================== ENTITY ============================= //
// =================================================================== //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32
}
impl Entity {
    pub(crate) fn new (id: u32) -> Self {
        Self { id }
    }

    pub fn id (self) -> u32 {
        self.id
    }

    pub fn from_id(id: u32) -> Self {
        Self { id }
    }
}

// =================================================================== //
// ============================== WORLD ============================== //
// =================================================================== //

use std::{any::{Any, TypeId}, collections::HashMap};

struct ComponentStorage {
    sparse: HashMap<u32, usize>,
    entitites: Vec<u32>,
    data: Vec<Box<dyn Any>>
}
impl ComponentStorage {
    fn new () -> Self {
        Self {
            sparse: HashMap::new(),
            entitites: Vec::new(),
            data: Vec::new()
        }
    }

    fn insert (&mut self, entity: Entity, component: Box<dyn Any>) {
        let id = entity.id();
        if let Some(&dense_idx) = self.sparse.get(&id) {
            self.data[dense_idx] = component;
        } else {
            let dense_idx = self.data.len();
            self.sparse.insert(id, dense_idx);
            self.entitites.push(id);
            self.data.push(component);
        }
    }

    fn get (&self, entity: Entity) -> Option<&dyn Any> {
        let dense_idx = *self.sparse.get(&entity.id())?;
        Some(self.data[dense_idx].as_ref())
    }

    fn get_mut (&mut self, entity: Entity) -> Option<&mut dyn Any> {
        let dense_idx = *self.sparse.get(&entity.id())?;
        Some(self.data[dense_idx].as_mut())
    }

    fn remove (&mut self, entity: Entity) {
        let id = entity.id();
        if let Some(dense_idx) = self.sparse.remove(&id) {
            let last_entity = *self.entitites.last().unwrap();
            self.entitites.swap_remove(dense_idx);
            self.data.swap_remove(dense_idx);

            if dense_idx < self.entitites.len() {
                self.sparse.insert(last_entity, dense_idx);
            }
        }
    }

    fn has (&self, entity: Entity) -> bool {
        self.sparse.contains_key(&entity.id())
    }
}

pub struct World {
    next_id: u32,
    entities: Vec<Entity>,
    storages: HashMap<TypeId, ComponentStorage>,
    resources: HashMap<TypeId, Box<dyn Any>>
}
impl World {
    pub fn new () -> Self {
        Self {
            next_id: 0,
            entities: Vec::new(),
            storages: HashMap::new(),
            resources: HashMap::new()
        }
    }

    // Entity management
    pub fn spawn (&mut self) -> Entity {
        let entity = Entity::new(self.next_id);
        self.next_id += 1;
        self.entities.push(entity);
        entity
    }

    pub fn despawn (&mut self, entity: Entity) {
        self.entities.retain(|e| e.id() != entity.id());
        for storage in self.storages.values_mut() {
            storage.remove(entity);
        }
    }

    pub fn entities (&self) -> &[Entity] {
        &self.entities
    }


    // Component management
    pub fn add_component<C: Component> (&mut self, entity: Entity, component: C) {
        self.storages
            .entry(TypeId::of::<C>())
            .or_insert_with(ComponentStorage::new)
            .insert(entity, Box::new(component));
    }

    pub fn remove_component<C: Component> (&mut self, entity: Entity) {
        if let Some(storage) = self.storages.get_mut(&TypeId::of::<C>()) {
            storage.remove(entity);
        }
    }

    pub fn get_component<C: Component> (&self, entity: Entity) -> Option<&C> {
        self.storages
            .get(&TypeId::of::<C>())?
            .get(entity)?
            .downcast_ref::<C>()
    }

    pub fn get_component_mut<C: Component> (&mut self, entity: Entity) -> Option<&mut C> {
        self.storages
            .get_mut(&TypeId::of::<C>())?
            .get_mut(entity)?
            .downcast_mut::<C>()
    }

    pub fn has_component<C: Component> (&self, entity: Entity) -> bool {
        self.storages
            .get(&TypeId::of::<C>())
            .map(|s| s.has(entity))
            .unwrap_or(false)
    }

    // Component query
    pub fn query<A: Component>(&self) -> impl Iterator<Item = (Entity, &A)> {
        let type_a = TypeId::of::<A>();
        self.entities.iter().filter_map(move |&entity| {
            let a = self
                .storages
                .get(&type_a)?
                .get(entity)?
                .downcast_ref::<A>()?;
            Some((entity, a))
        })
    }

    pub fn query2<A: Component, B: Component>(&self) -> impl Iterator<Item = (Entity, &A, &B)> {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        self.entities.iter().filter_map(move |&entity| {
            let a = self.storages.get(&type_a)?.get(entity)?.downcast_ref::<A>()?;
            let b = self.storages.get(&type_b)?.get(entity)?.downcast_ref::<B>()?;
            Some((entity, a, b))
        })
    }

    pub fn query3<A: Component, B: Component, C: Component>(&self) -> impl Iterator<Item = (Entity, &A, &B, &C)> {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let type_c = TypeId::of::<C>();
        self.entities.iter().filter_map(move |&entity| {
            let a = self.storages.get(&type_a)?.get(entity)?.downcast_ref::<A>()?;
            let b = self.storages.get(&type_b)?.get(entity)?.downcast_ref::<B>()?;
            let c = self.storages.get(&type_c)?.get(entity)?.downcast_ref::<C>()?;
            Some((entity, a, b, c))
        })
    }

    // Resources Management
    pub fn insert_resource<R: 'static> (&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn get_resource<R: 'static> (&self) -> Option<&R> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref::<R>()
    }

    pub fn get_resource_mut<R: 'static> (&mut self) -> Option<&mut R> {
        self.resources.get_mut(&TypeId::of::<R>())?.downcast_mut::<R>()
    }
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================== //
// ============================== SYSTEM ============================= //
// =================================================================== //

pub trait System {
    fn run(&mut self, world: &mut World);
}

pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}
impl Scheduler {
    pub fn new () -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn add_system<S: System + 'static> (&mut self, system: S) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run (&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}
impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
