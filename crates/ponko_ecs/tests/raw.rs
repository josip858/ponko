use ponko_ecs::World;
#[test]
fn raw_views_share_storage_and_release_the_exclusive_borrow() {
    let mut world = World::new();
    let e = world.spawn((7u32,));
    let id = world.component_id::<u32>().unwrap();
    {
        let mut raw = world.raw();
        assert!(raw.get_dynamic(e, id).is_some());
        assert_eq!(raw.component_by_name("u32"), Some(id));
        let mut rows = 0;
        raw.for_each_chunk(&[id], |chunk| rows += chunk.len());
        assert_eq!(rows, 1);
        assert!(raw.remove_dynamic(e, id));
    }
    assert!(world.get::<u32>(e).is_none());
    assert!(!world.raw_ref().archetypes().is_empty());
}

#[test]
#[allow(deprecated)]
fn the_old_world_spellings_forward_to_the_raw_view() {
    let mut world = World::new();
    let e = world.spawn((7u32,));
    let id = world.component_id::<u32>().unwrap();
    assert_eq!(world.get_dynamic(e, id), world.raw_ref().get_dynamic(e, id));
    assert_eq!(world.component_by_name("u32"), Some(id));
    assert_eq!(world.archetypes().len(), world.raw_ref().archetypes().len());
    let mut rows = 0;
    world.for_each_chunk(&[id], |chunk| rows += chunk.len());
    assert_eq!(rows, 1);
    let score = [3u32, 1];
    // SAFETY: points to the bytes of a [u32; 2].
    unsafe { world.insert_resource_dynamic("Score", std::alloc::Layout::new::<[u32; 2]>(), score.as_ptr().cast()) };
    assert_eq!(
        world.resource_layout_dynamic("Score"),
        world.raw_ref().resource_layout_dynamic("Score")
    );
    assert!(world.get_resource_dynamic("Score").is_some());
    assert!(world.remove_resource_dynamic("Score"));
    assert!(world.remove_dynamic(e, id));
    assert!(world.get::<u32>(e).is_none());
}

#[test]
fn a_shared_raw_view_hands_out_borrows_of_the_world() {
    let mut world = World::new();
    world.spawn((1u8,));
    // The slice outlives the temporary view: it borrows the world itself.
    let tables = world.raw_ref().archetypes();
    assert!(!tables.is_empty());
}
