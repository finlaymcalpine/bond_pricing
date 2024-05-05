pub struct SimpleBond {
    pub face_value: f32,
    pub coupon: f32,
    pub frequency: f32,
    pub maturity: f32,
    pub yield_to_maturity: f32,
    pub price: f32,
}

impl SimpleBond {
    // Need to refactor this function, to make the pv calcs more readable
    pub fn price(&mut self) -> f32 {
        let pv_principal: f32 = self.face_value
            / ((1.0 + (self.yield_to_maturity / self.frequency))
                .powf(self.maturity * self.frequency));

        let pv_coupon: f32 = ((self.coupon * self.face_value) / self.yield_to_maturity)
            * (1.0
                - (1.0
                    / ((1.0 + (self.yield_to_maturity / self.frequency))
                        .powf(self.maturity * self.frequency))));

        self.price = pv_principal + pv_coupon;

        self.price
    }

    // Still need to handle non-convergence. Newton Method comes from here: https://en.wikipedia.org/wiki/Newton%27s_method#Code
    pub fn solve_yield_to_maturity(&mut self, mut x0: f32, iter: i32, tolerance: f32, epsilon: f32) -> f32 {
        let max_iter: i32 = iter;
        let coupon_periods = self.frequency * self.maturity;

        for _ in 1..max_iter {

            let y = (self.face_value - ((self.coupon * self.face_value) / x0))
                * ((1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods))
                + ((self.coupon * self.face_value) / x0)
                - self.price;
            
            let y_prime = ((self.coupon * self.face_value) / x0.powf(2.0))
                * (1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods)
                - ((self.face_value - ((self.coupon * self.face_value) / x0))
                    * (coupon_periods
                        * (1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods - 1.0))
                    / self.frequency)
                - ((self.coupon * self.face_value) * x0.powf(-2.0));

            if y_prime.abs() < epsilon {
                break;
            };

            let x1: f32 = x0 - (y / y_prime);

            if (x1 - x0).abs() <= tolerance {
                self.yield_to_maturity = x1;
                break;
            }
            x0 = x1;
        }

        self.yield_to_maturity
    }

    // this doesn't handle the case where we've given the yield to maturity of a bond, but not the correct price
    pub fn current_yield(&self) -> f32 {
        let current_yield = (self.face_value * self.coupon) / self.price * 100.0;
        current_yield
    }

    // using the explicit formula for duration here, as given in Luenberger
    pub fn macauly_duration(&self) -> f32 {
        let n = self.maturity * self.frequency;
        let c = self.coupon / 2.0;
        let y = self.yield_to_maturity / 2.0;
        let duration = (1.0 + y) / (self.frequency * y)
            - (1.0 + y + (n * (c - y)))
            / (self.frequency * c * ((1.0 + y).powf(n) - 1.0) + self.frequency * y);
        duration
    }

    pub fn modified_duration(&self) -> f32 {
        let modified = self.macauly_duration() / (1.0 + (self.yield_to_maturity / self.frequency));
        modified
    }

    pub fn price_sensitivity(&self, old_yield: f32, new_yield: f32) -> f32 {
        let yield_change = new_yield - old_yield;
        let derivative = -1.0 * self.modified_duration() * self.price;
        derivative * yield_change
    }
}