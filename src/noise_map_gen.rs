use perlin_noise::*;

pub fn generate_noise_map(map_width: usize, map_height: usize, mut scale: f64) -> Vec<Vec<f64>> {
    let mut noise_map: Vec<Vec<f64>> = vec![vec![0.0; map_width]; map_height];
    let perlin = PerlinNoise::new();
    if (scale == 0.0) {
        scale = 0.0001;
    }

    // for debugging only
    let mut perlin_min = 1.0;
    let mut perlin_max = 0.0;

    for y in 0..map_height {
        for x in 0..map_width {
            let sample_x = x as f64 / scale;
            let sample_y = y as f64 / scale;

            noise_map[y][x] = perlin.get2d([sample_x, sample_y]);

            if noise_map[y][x] < perlin_min {perlin_min = noise_map[y][x]}
            if noise_map[y][x] > perlin_max {perlin_max = noise_map[y][x]}
        }
    }

    println!("Perlin range: {} - {}", perlin_min, perlin_max);
    noise_map
}
