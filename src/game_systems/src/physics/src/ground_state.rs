use bevy_ecs::prelude::Query;
use temper_components::player::grounded::OnGround;

pub fn snapshot(mut query: Query<&mut OnGround>) {
    for mut grounded in query.iter_mut() {
        grounded.snapshot();
    }
}
