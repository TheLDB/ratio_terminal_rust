pub struct RatioHandler;
use rand::Rng;

impl RatioHandler {
    // Cant be async cuz of some safe threading thing, so it lives here... alone
    pub fn gen_random() -> i32 {
        let mut rng = rand::thread_rng();
        let det = rng.gen_range(0..2);
        return det;
    }
}