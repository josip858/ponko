//! Dimension modules preserve component identity, serialized names and query behavior.
use ponko::prelude::*;
mod common;

#[cfg(feature = "physics")]
#[test]
fn filtered_2d_queries_exclude_entities_and_sensors_without_changing_legacy_calls() {
    use ponko::d2::*;
    Game::new("queries", 1, 1)
        .physics(Vec2::ZERO)
        .sim_system(|ctx| {
            let solid = ctx.world.resource::<Entity>();
            let hit: Hit = ctx
                .physics
                .cast_ray_filtered(Vec2::ZERO, Vec2::X, 100.0, QueryFilter::default())
                .unwrap();
            assert_eq!(hit.entity, *solid);
            assert_eq!(hit.distance, 9.0);
            assert_eq!(hit.point, vec2(9.0, 0.0));
            assert_eq!(hit.normal, Vec2::NEG_X);
            assert!(
                ctx.physics
                    .cast_ray_filtered(Vec2::ZERO, Vec2::X, 100.0, QueryFilter::default().exclude(*solid))
                    .is_none()
            );
            assert_eq!(ctx.physics.cast_ray(Vec2::ZERO, Vec2::X, 100.0).unwrap().distance, 4.0);
            let mut found = Vec::new();
            ctx.physics
                .overlap_ball(vec2(5.0, 0.0), 1.0, QueryFilter::default(), &mut found);
            assert!(found.is_empty());
            ctx.physics
                .overlap_ball(vec2(5.0, 0.0), 1.0, QueryFilter::default().with_sensors(), &mut found);
            assert_eq!(found.len(), 1);
            // The legacy wrappers keep seeing sensors, as before.
            ctx.physics.entities_in_circle(vec2(5.0, 0.0), 1.0, &mut found);
            assert_eq!(found.len(), 1);
            ctx.physics.entities_at(vec2(5.0, 0.0), &mut found);
            assert_eq!(found.len(), 1);
            ctx.physics
                .overlap_point(vec2(5.0, 0.0), QueryFilter::default(), &mut found);
            assert!(found.is_empty());
            assert!(ctx.physics.mass(*solid).is_some());
            assert!(ctx.physics.apply_impulse(*solid, Vec2::ZERO));
        })
        .run_headless(1, |ctx| {
            let solid = ctx.world.spawn((
                Transform::at(vec2(10.0, 0.0)),
                RigidBody::fixed(),
                Collider::circle(1.0),
            ));
            ctx.world.insert_resource(solid);
            ctx.world.spawn((
                Transform::at(vec2(5.0, 0.0)),
                RigidBody::fixed(),
                Collider::circle(1.0).sensor(),
            ));
        });
}

#[cfg(feature = "physics3d")]
#[test]
fn filtered_3d_queries_use_the_same_spelling_and_hit_fields() {
    #[cfg(feature = "3d")]
    use ponko::d3::*;
    Game::new("queries", 1, 1)
        .physics3d(Vec3::ZERO)
        .sim_system(|ctx| {
            let solid = *ctx.world.resource::<Entity>();
            let hit: Hit = ctx
                .physics3d
                .cast_ray_filtered(Vec3::ZERO, Vec3::X, 100.0, QueryFilter::default())
                .unwrap();
            assert_eq!(hit.entity, solid);
            assert_eq!(hit.distance, 9.0);
            assert_eq!(hit.point, vec3(9.0, 0.0, 0.0));
            assert_eq!(hit.normal, Vec3::NEG_X);
            assert!(
                ctx.physics3d
                    .cast_ray_filtered(Vec3::ZERO, Vec3::X, 100.0, QueryFilter::default().exclude(solid))
                    .is_none()
            );
            assert_eq!(
                ctx.physics3d
                    .cast_ray(Vec3::ZERO, Vec3::X, 100.0, QueryFilter::default())
                    .unwrap(),
                hit
            );
            let mut found = Vec::new();
            ctx.physics3d
                .overlap_ball(vec3(5.0, 0.0, 0.0), 1.0, QueryFilter::default(), &mut found);
            assert!(found.is_empty());
            ctx.physics3d.overlap_ball(
                vec3(5.0, 0.0, 0.0),
                1.0,
                QueryFilter::default().with_sensors(),
                &mut found,
            );
            assert_eq!(found.len(), 1);
            assert!(ctx.physics3d.mass(solid).is_some());
            assert!(ctx.physics3d.apply_impulse(solid, Vec3::ZERO));
        })
        .run_headless(1, |ctx| {
            let solid = ctx.world.spawn((
                Transform::at(vec3(10.0, 0.0, 0.0)),
                RigidBody::fixed(),
                Collider::sphere(1.0),
            ));
            ctx.world.insert_resource(solid);
            ctx.world.spawn((
                Transform::at(vec3(5.0, 0.0, 0.0)),
                RigidBody::fixed(),
                Collider::sphere(1.0).sensor(),
            ));
        });
}

#[cfg(feature = "3d")]
#[test]
fn prefab_3d_placement_preserves_identity_rotation_scale_and_saved_names() {
    #[cfg(feature = "3d")]
    use ponko::d3::*;

    let file = common::TempAsset::new("dimension-prefab.json");
    Game::new("prefab", 1, 1).run_headless(0, |ctx| {
        let source = Transform {
            rotation: Quat::from_rotation_y(0.25),
            scale: Vec3::splat(2.0),
            ..Default::default()
        };
        let e = ctx.world.spawn((source,));
        ctx.assets.save_prefab(&ctx.world, e, file.name()).unwrap();
        let at = Transform::at(vec3(4.0, 5.0, 6.0)).rotated(Quat::from_rotation_y(0.5));
        let placed = ctx.spawn_prefab(file.name(), at).unwrap();
        assert_eq!(ctx.world.get::<Transform>(placed), Some(&at));
        let positioned = ctx.spawn_prefab(file.name(), vec3(7.0, 8.0, 9.0)).unwrap();
        assert_eq!(
            ctx.world.get::<Transform>(positioned).unwrap().position,
            vec3(7.0, 8.0, 9.0)
        );
        assert_eq!(
            ctx.world.get::<Transform>(positioned).unwrap().rotation,
            source.rotation
        );
        assert_eq!(ctx.world.get::<Transform>(positioned).unwrap().scale, source.scale);
        assert!(ctx.world.save().to_string().contains("Transform3D"));
    });
}

#[test]
fn prefab_2d_transform_placement_sets_rotation_and_scale_as_well_as_position() {
    use ponko::d2::Transform;
    let file = common::TempAsset::new("dimension-prefab-2d.json");
    Game::new("prefab", 1, 1).run_headless(0, |ctx| {
        let source = Transform::at(vec2(1.0, 2.0)).rotated(0.25);
        let e = ctx.world.spawn((source,));
        ctx.assets.save_prefab(&ctx.world, e, file.name()).unwrap();
        let position = ctx.spawn_prefab(file.name(), vec2(3.0, 4.0)).unwrap();
        assert_eq!(ctx.world.get::<Transform>(position).unwrap().rotation, source.rotation);
        let at = Transform {
            position: vec2(5.0, 6.0),
            rotation: 0.75,
            scale: vec2(2.0, 3.0),
        };
        let placed = ctx.spawn_prefab(file.name(), at).unwrap();
        assert_eq!(ctx.world.get::<Transform>(placed), Some(&at));
    });
}
