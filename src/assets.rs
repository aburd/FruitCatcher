use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::actors::{Actor, ActorType};

pub struct Assets {
    player_image: graphics::Image,
    fruit_image: graphics::Image,
    shot_sound: audio::SpatialSource,
    hit_sound: audio::SpatialSource,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let fruit_image = graphics::Image::new(ctx, "/fruit.png")?;

        let mut shot_sound = audio::SpatialSource::new(ctx, "/pew.ogg")?;
        let mut hit_sound = audio::SpatialSource::new(ctx, "/boom.ogg")?;

        shot_sound.set_ears([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        hit_sound.set_ears([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);

        Ok(Assets {
            player_image,
            fruit_image,
            shot_sound,
            hit_sound,
        })
    }

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Player => &mut self.player_image,
            ActorType::Fruit => &mut self.fruit_image,
        }
    }
}