#![cfg(feature = "physics3d")]
#[allow(unused_imports)]
use ponko::d3::{
    Collider as Collider3D, Joint as Joint3D, JointAxis as JointAxis3D, JointDrive as JointDrive3D,
    JointMotion as JointMotion3D, RigidBody as RigidBody3D, Transform as Transform3D,
};
use ponko::prelude::*;

#[test]
fn guide_joint_and_dominance_snippets_compile_and_step() {
    Game::new("joints", 1, 1).physics3d(Vec3::ZERO).run_headless(5, |ctx| {
        // ANCHOR: joints
        let a = ctx
            .world
            .spawn((Transform3D::default(), RigidBody3D::fixed(), Collider3D::sphere(0.1)));
        let b = ctx.world.spawn((
            Transform3D::at(vec3(0.0, -1.0, 0.0)),
            RigidBody3D::dynamic(),
            Collider3D::sphere(0.2),
        ));
        let mut joint = Joint3D::spherical(a, b)
            .motion(JointAxis3D::AngularX, JointMotion3D::Limited(-0.785, 0.785))
            .swing_cone(0.524)
            .drive(JointAxis3D::AngularX, JointDrive3D::spring(0.0, 100.0, 20.0));
        joint.frame_b.position = Vec3::Y;
        joint.contacts_enabled = false;
        let connection = ctx.world.spawn((joint,));
        // Despawn `connection` to detach; either body's despawn also removes the joint.
        // ANCHOR_END: joints
        assert!(ctx.world.get::<Joint3D>(connection).is_some());
        // ANCHOR: dominance
        let pusher = RigidBody3D::dynamic().dominance(1);
        let pushed = RigidBody3D::dynamic(); // group 0
        // Both remain dynamic: gravity, walls and equal-group bodies still affect them.
        ctx.world
            .spawn((Transform3D::at(Vec3::X * 3.0), pusher, Collider3D::cuboid(Vec3::ONE)));
        ctx.world
            .spawn((Transform3D::at(Vec3::X * 4.0), pushed, Collider3D::cuboid(Vec3::ONE)));
        // ANCHOR_END: dominance
    });
}
