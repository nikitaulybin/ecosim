use crate::prelude::*;

pub struct VegetationPlugin;

const INITIAL_PLANT_COUNT: i32 = 50;

#[derive(Component)]
pub struct Plant {
    growth_period: i32,
    plant_type: PlantType,
}

#[derive(Component)]
pub struct ExistenceTimer(Timer);

#[derive(Component)]
pub struct GrowthRate(f32);

#[derive(EnumIter)] 
pub enum PlantType {
    Anthurium,
    Daffidoil,
    Foxglove,
    Ginger,
    Heather,
    Pansies,
    Proteam,
    Scarletstar, 
}

impl PlantType {
    pub fn spawn_chance(&self) -> f32 {
        match self {
            PlantType::Anthurium => 6.25,
            PlantType::Daffidoil => 12.5,
            PlantType::Foxglove => 12.5,
            PlantType::Ginger => 12.5,
            PlantType::Heather => 12.5,
            PlantType::Pansies => 6.25,
            PlantType::Proteam => 25.0,
            PlantType::Scarletstar => 12.5,
        }
    }

    pub fn cdf_array() -> Vec<f32>{
        let mut cdf_array: Vec<f32> = Vec::new();
        let mut acc: f32 = 0.0;
        for plant_type in PlantType::iter() {
            acc += plant_type.spawn_chance();
            cdf_array.push(acc);
        }

        cdf_array
    }
}

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        let cdf_array = PlantType::cdf_array();
        app.insert_resource(resource)
    }
}

impl VegetationPlugin { 
    fn spawn_initial_plants(mut commands: Commands, map: Res<Map>) {
        // let mut plant_counter = 0;
        // let mut rng = thread_rng();
        // while plant_counter < INITL_PLANT_COUNT {
        //     let plant_pos_idx = rng.gen_range(0..map.size());
        //     if map.tiles[plant_pos_idx].is_traversable() {
        //         commands.spawn_bundle((
        //             Plant,
        //             ExistenceTimer(Timer::fro
        //         ))
        //     }
        // }
    }
}
