use bevy::prelude::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = Visuals)]
pub struct VisualOf(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = VisualOf)]
pub struct Visuals(Vec<Entity>);

#[derive(Component, Debug)]
#[require(Transform, Visibility, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct VisualBase;

// To render a new thing, we need a VisualKind enum entry (e.g. Tile) so
// the visual system can decide how to add the visuals for that entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedVisualComponents {
    // crate::board::components::TileData,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct SpawnVisualRequest(Entity);

#[derive(Component)]
pub struct HasVisual;

type Spawner = Box<dyn Fn(Entity, &dyn Any, &mut Commands) + Send + Sync>;

#[derive(Default, Resource)]
pub struct VisualRegistry {
    map: HashMap<TypeId, Spawner>,
}

impl VisualRegistry {
    pub fn register<T: Component + 'static, F>(&mut self, f: F)
    where
        F: Fn(Entity, &T, &mut Commands) + Send + Sync + 'static,
    {
        let wrapper: Spawner = Box::new(move |e, any, commands| {
            let comp = any.downcast_ref::<T>().expect("type mismatch");
            f(e, comp, commands);
        });
        self.map.insert(TypeId::of::<T>(), wrapper);
    }

    pub fn get(&self, id: TypeId) -> Option<&Spawner> {
        self.map.get(&id)
    }
}

// Component bundles for common visual entities
