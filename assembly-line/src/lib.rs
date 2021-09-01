pub fn production_rate_per_hour(speed: u8) -> f64 {
    const LOW_SPEED_PRODUCTION: f64 = 221f64;
    let success_rate = match speed {
        1..=4 => 1f64,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0f64,
    };

    success_rate * (LOW_SPEED_PRODUCTION * speed as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60u32
}
