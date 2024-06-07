use rand::Rng;

pub fn random(surface: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();

    loop {
        let length: f64 = rng.gen_range((surface / 5.0).sqrt()..(surface * 3.0 / 2.0).sqrt());
        let width: f64 = surface / length;
        if length / width <= 4.0 && width / length <= 4.0 {
            if rng.gen_bool(0.5) {
                return (length, width);
            } else {
                return (width, length);
            }
        }
    }
}

pub fn un_sur_deux() -> bool {
    let mut rng = rand::thread_rng();

    rng.gen_bool(0.5)
}
