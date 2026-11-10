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
