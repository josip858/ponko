//! Offscreen benchmark of what the 3D renderer uploads per frame: bytes by
//! kind, buffers made, and the CPU time of preparing the frame, for a few
//! mostly static scenes. Run explicitly, in release:
//! `cargo test --release --lib --features 3d upload_bench -- --ignored --nocapture`.
use super::buffer_render_test::Rig;
use super::traffic;
use crate::*;

/// `n` × `n` cubes on the ground under a camera, with sun shadows.
fn ground(n: i32) -> Option<(Rig, Vec<Entity>, Mesh)> {
    let mut rig = Rig::new()?;
    let mesh = rig.mesh(
        &MeshData::cube(0.4),
        MaterialDesc {
            color: Color::WHITE,
            ..Default::default()
        },
    );
    let mut cubes = Vec::new();
    for x in 0..n {
        for z in 0..n {
            let at = vec3(x as f32 - n as f32 / 2.0, 0.0, z as f32 - n as f32 / 2.0);
            cubes.push(rig.ctx.world.spawn((Transform3D::at(at), Model::new(mesh))));
        }
    }
    rig.ctx.camera3d.position = vec3(0.0, n as f32 * 0.6, n as f32 * 0.6);
    rig.ctx.camera3d.look_at(Vec3::ZERO);
    Some((rig, cubes, mesh))
}

/// Draws `frames` frames after 10 warm-up ones with the clock running,
/// calling `change` before each; prints the averages per frame.
fn measure(name: &str, rig: &mut Rig, frames: usize, mut change: impl FnMut(&mut Rig, usize)) {
    for i in 0..10 {
        rig.time = (i as f32 / 60.0, 1.0 / 60.0);
        change(rig, i);
        rig.draw();
    }
    traffic::take();
    let mut total = GpuUploads::default();
    let mut prepare = Vec::with_capacity(frames);
    for i in 10..10 + frames {
        rig.time = (i as f32 / 60.0, 1.0 / 60.0);
        change(rig, i);
        rig.draw();
        prepare.push(rig.prepare_ms);
        let c = traffic::take();
        total.instances += c.instances;
        total.uniforms += c.uniforms;
        total.lights += c.lights;
        total.static_batches += c.static_batches;
        total.skins += c.skins;
        total.other += c.other;
        total.buffers_created += c.buffers_created;
        total.buffer_bytes_created += c.buffer_bytes_created;
    }
    prepare.sort_by(f64::total_cmp);
    let n = frames as u64;
    println!(
        "{name:<34} {:>9} B/frame  (instances {:>7}, uniforms {:>6}, other {:>6})  buffers made {:>3}  prepare p50 {:.3} ms",
        (total.instances + total.uniforms + total.lights + total.static_batches + total.skins + total.other) / n,
        total.instances / n,
        total.uniforms / n,
        total.other / n,
        total.buffers_created,
        prepare[frames / 2],
    );
}

#[test]
#[ignore = "requires a GPU; run explicitly in release for numbers"]
fn upload_benchmark() {
    const FRAMES: usize = 120;
    let Some((mut rig, _, _)) = ground(32) else { return };
    measure("still, 1024 models", &mut rig, FRAMES, |_, _| {});

    let Some((mut rig, cubes, _)) = ground(32) else { return };
    rig.ctx.post.anti_aliasing = AntiAliasing::Taa;
    measure("1% moving, 1024 models, TAA", &mut rig, FRAMES, |rig, i| {
        for &e in cubes.iter().step_by(100) {
            rig.ctx.world.get_mut::<Transform3D>(e).unwrap().position.y = (i % 20) as f32 * 0.02;
        }
    });

    let Some((mut rig, _, _)) = ground(32) else { return };
    measure("camera panning, 1024 models", &mut rig, FRAMES, |rig, i| {
        let x = (i as f32 * 0.05).sin() * 6.0;
        rig.ctx.camera3d.position = vec3(x, 19.0, 19.0);
        rig.ctx.camera3d.look_at(vec3(x, 0.0, 0.0));
    });

    let Some((mut rig, mut cubes, mesh)) = ground(32) else {
        return;
    };
    measure("5% respawned a frame, 1024 models", &mut rig, FRAMES, |rig, i| {
        for e in cubes.drain(..50) {
            rig.ctx.world.despawn(e);
        }
        for k in 0..50 {
            let at = vec3((k % 10) as f32 - 5.0, 0.0, (k / 10) as f32 - 5.0 + (i % 4) as f32);
            cubes.push(rig.ctx.world.spawn((Transform3D::at(at), Model::new(mesh))));
        }
    });

    let Some((mut rig, _, _)) = ground(16) else { return };
    for k in 0..8 {
        let at = Transform3D::at(vec3(k as f32 - 4.0, 1.0, 0.0));
        let mut e = ParticleEmitter3D::new(ParticleSettings3D {
            rate: 0.0,
            burst: 250,
            max_particles: 250,
            lifetime: (1e4, 1e4),
            ..Default::default()
        });
        e.update(1.0 / 60.0, &at);
        rig.ctx.world.spawn((at, e));
        let line = Line3D::new(
            vec![vec3(-4.0, 0.5, k as f32), vec3(4.0, 0.5, k as f32)],
            0.05,
            Color::RED,
        );
        rig.ctx.world.spawn((Transform3D::default(), line));
    }
    measure("still: 2000 particles, 8 lines", &mut rig, FRAMES, |_, _| {});
}
