use rand::Rng;

pub struct Coin {}

impl Coin {
    pub fn toss(probability: f32) -> bool {
        if probability >= 1.0 {
            return true;
        }

        if probability <= 0.0 {
            return false;
        }

        let random_number = rand::thread_rng().gen_range(0.0, 1.0);

        probability > random_number
    }
}
