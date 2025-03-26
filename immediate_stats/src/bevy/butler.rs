#[cfg(test)]
mod tests {
    use crate::StatContainer;
    use crate::stat::Stat;
    use bevy_app::App;
    use bevy_butler::*;
    use bevy_ecs::prelude::*;
    use bevy_reflect::prelude::*;
    use immediate_stats_macros::StatContainer;

    #[butler_plugin]
    struct MyPlugin;

    #[derive(Reflect, Resource, StatContainer, Default, PartialEq, Debug)]
    #[resource(plugin = MyPlugin)]
    struct Health(Stat);

    #[test]
    fn reset_resource_auto() {
        let mut app = App::new();

        app.add_plugins(MyPlugin);

        app.insert_resource(Health(Stat {
            base: 100,
            bonus: 50,
            multiplier: 2.0,
        }));

        app.update();

        assert_eq!(*app.world().resource::<Health>(), Health(Stat::new(100)));
    }
}
