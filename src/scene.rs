// use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
// use std::{fs::File, io::Write};
//
// use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
// use crate::components::{
//     MainCamera, 
//     Cell, Player, Direction, Hostile, Enemy, Lifespan,
//     Explosion, Virus, Particle, Wall};
//
// pub struct ScenePlugin;
//
// impl Plugin for ScenePlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .register_type::<Wall>()
//             .register_type::<Option<Rect>>()
//             .register_type::<Option<Vec2>>()
//             .register_type_data::<Option<Vec2>, ReflectSerialize>()
//             .register_type_data::<Option<Vec2>, ReflectDeserialize>()
//             .register_type_data::<Sprite, ReflectComponent>()
//             .register_type::<Sprite>()
//             .add_startup_system(save_scene)
//             .add_startup_system(load_scene.after(save_scene))
//             .add_system(log)
//             .run();
//     }
// }
//
// fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) { 
//     commands.spawn(DynamicSceneBundle {
//         scene: asset_server.load("scenes/wall.scn.ron"),
//          
//         ..default()
//     });
// }
//
// fn save_scene(world: &mut World) {
//     let size = 20.0;
//     let top_translation = Vec3::new(0.0, SCREEN_HEIGHT / 2.0 - (size / 2.0), 1.0);
//     let bottom_translation = Vec3::new(0.0, -SCREEN_HEIGHT / 2.0 + (size / 2.0), 1.0);
//     let left_translation = Vec3::new(-SCREEN_WIDTH / 2.0 + (size / 2.0), 0.0, 1.0);
//     let right_translation = Vec3::new(SCREEN_WIDTH / 2.0 - (size / 2.0), 0.0, 1.0);
//     world.spawn((get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, top_translation), Wall));
//     world.spawn((get_rectangle(Color::DARK_GRAY, SCREEN_WIDTH, size, bottom_translation), Wall));
//     world.spawn((get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, left_translation), Wall));
//     world.spawn((get_rectangle(Color::DARK_GRAY, size, SCREEN_HEIGHT, right_translation), Wall));
//     let type_registry = world.resource::<AppTypeRegistry>();
//     let scene = DynamicScene::from_world(&world, type_registry);
//     let serialized_scene = scene.serialize_ron(type_registry).unwrap();
//     info!("=================================================================");
//     info!("{}", serialized_scene);
//
//     #[cfg(not(target_arch = "wasm32"))]
//     IoTaskPool::get().spawn(async move {
//         File::create("assets/scenes/box.scn.ron")
//             .and_then(|mut file| file.write(serialized_scene.as_bytes()))
//             .expect("Error writing scene to file");
//     }).detach();
// }
//
// fn get_rectangle(color: Color, height: f32, width: f32, translation: Vec3) -> SpriteBundle {
//     SpriteBundle {
//         sprite: Sprite {
//             color,
//             ..default()
//         },
//         transform: Transform::from_scale(Vec3::new(height, width, 1.0))
//             .with_translation(translation),
//         ..default()
//     }
// }
//
// fn log(query: Query<(Entity, &Wall), Changed<Wall>>) { 
//     for (entity, wall) in &query {
//         info!("  Wall Entity({})", entity.index());
//     }
// }
