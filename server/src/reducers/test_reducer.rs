// use crate::entity;
// use crate::{Entity, EntityType};
// use spacetimedb::{reducer, ReducerContext, Table};

// #[reducer]
// pub fn create_test_entity(ctx: &ReducerContext, name: String) {
//     log::info!("Creating test entity: {}", name);

//     let new_entity = Entity {
//         id: 0,
//         identity: None,
//         entity_type: EntityType::Player,
//         name,
//         description: "A test entity".to_string(),
//         room_id: 1,
//         x: 0.0,
//         y: 0.0,
//         z: 0.0,
//         volume: 1.0,
//         weight: 70.0,
//         max_capacity: 50.0,
//         hp: 100,
//         max_hp: 100,
//         stamina: 100,
//         max_stamina: 100,
//         mana: 100,
//         max_mana: 100,
//         dexterity: 100,
//         strength: 100,
//         vitality: 100,
//         perception: 100,
//         willpower: 100,
//         is_alive: true,
//         is_active: true,
//         created_at: 0,
//         last_action_at: 0,
//     };

//     ctx.db.entity().try_insert(new_entity);

//     log::info!("Entity inserted successfully!");
// }
