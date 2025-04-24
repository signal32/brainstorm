use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Animates sprites from their sprite sheet.
/// 1x1 sprite sheets (i.e. static images) are acceptable,
/// allowing for mixture of static and animated sprites on
/// the same kind of asset.
/// 
/// Relies upon [atlas_dimensions] existing in config file.
/// If they do not exist, it will default to static image.
pub fn animate_sys(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // static image
                if indices.first == indices.last {
                    atlas.index = 1;
                    continue;
                }

                // standard animation loop
                if atlas.index >= indices.last {
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
