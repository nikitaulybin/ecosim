use perlin_noise::*;

pub fn generate_noise_map(
    map_width: usize,
    map_height: usize,
    mut scale: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> Vec<Vec<f64>> {
    let mut noise_map: Vec<Vec<f64>> = vec![vec![0.0; map_width]; map_height];
    let perlin = PerlinNoise::new();
    if scale == 0.0 {
        scale = 0.0001;
    }

    let mut perlin_min = f64::MAX;
    let mut perlin_max = f64::MIN;

    for y in 0..map_height {
        for x in 0..map_width {
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut noise_height = 0.0;
            for _octave in 0..octaves {
                let sample_x = x as f64 / scale * frequency;
                let sample_y = y as f64 / scale * frequency;

                let noise_value = perlin.get2d([sample_x, sample_y]) * 2.0 - 1.0;
                noise_height += noise_value * amplitude;

                amplitude *= persistence;
                frequency *= lacunarity;

                if noise_height < perlin_min {
                    perlin_min = noise_height;
                }
                if noise_height > perlin_max {
                    perlin_max = noise_height;
                }
            }
            noise_map[y][x] = noise_height;
        }
    }

    for y in 0..map_height {
        for x in 0..map_width {
            noise_map[y][x] = inv_lerp(perlin_min, perlin_max, noise_map[y][x]);
        }
    }
    println!("Perlin range: {} - {}", perlin_min, perlin_max);
    noise_map
}

fn inv_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}
