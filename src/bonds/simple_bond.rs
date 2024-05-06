/// SimpleBond represents a basic coupon bond with complete coupon periods (i.e. no accrued interest).
///
/// At present, any SimpleBond needs to have a price and yield given as a float. Since we may want to calculate one of these, they can be given as 0.0
/// Also, the bond needs to have full coupon periods remaining until maturity. It does not adjust for accrued interest when a bond has incomplete coupon periods.
pub struct SimpleBond {
    pub face_value: f32,
    pub coupon: f32,
    pub frequency: f32,
    pub maturity: f32,
    pub yield_to_maturity: f32,
    pub price: f32,
}

impl SimpleBond {
    /// Solves the pricing equation for a given bond.
    pub fn solve_price(&mut self) -> f32 {
        // For shorter code, we create these values as variables in the function.
        let coupon_periods = self.frequency * self.maturity;
        let coupon_amount = self.coupon * self.face_value;

        let pv_principal: f32 = self.face_value
            / ((1.0 + (self.yield_to_maturity / self.frequency)).powf(coupon_periods));

        let pv_coupon: f32 = ((coupon_amount) / self.yield_to_maturity)
            * (1.0
                - (1.0 / ((1.0 + (self.yield_to_maturity / self.frequency)).powf(coupon_periods))));

        self.price = pv_principal + pv_coupon;

        self.price
    }

    /// Solves the yield to maturity calculation for a given bond, using Newton's Method.
    /// Newton Method comes from here: https://en.wikipedia.org/wiki/Newton%27s_method#Code
    /// The function still needs to be re-factored for cases of non-convergence. Specifically, it needs to produce an error in that case.
    pub fn solve_yield_to_maturity(
        &mut self,
        mut x0: f32,
        iter: i32,
        tolerance: f32,
        epsilon: f32,
    ) -> f32 {
        let max_iter: i32 = iter;
        // For shorter code, we create these values as variables in the function.
        let coupon_periods = self.frequency * self.maturity;
        let coupon_amount = self.coupon * self.face_value;

        for _ in 1..max_iter {
            let y = (self.face_value - ((coupon_amount) / x0))
                * ((1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods))
                + ((coupon_amount) / x0)
                - self.price;

            let y_prime = ((coupon_amount) / x0.powf(2.0))
                * (1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods)
                - ((self.face_value - ((coupon_amount) / x0))
                    * (coupon_periods
                        * (1.0 + (x0 / self.frequency)).powf(-1.0 * coupon_periods - 1.0))
                    / self.frequency)
                - ((coupon_amount) * x0.powf(-2.0));

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

    /// this doesn't handle the case where we've given the yield to maturity of a bond, but not the correct price
    pub fn current_yield(&self) -> f32 {
        let current_yield = (self.face_value * self.coupon) / self.price * 100.0;
        current_yield
    }

    /// Finds the Macauly Duration of the bond, using the explicit formula for duration as given in Luenberger, pg. 58.
    pub fn macauly_duration(&self) -> f32 {
        let n = self.maturity * self.frequency;
        let c = self.coupon / 2.0;
        let y = self.yield_to_maturity / 2.0;
        let duration = (1.0 + y) / (self.frequency * y)
            - (1.0 + y + (n * (c - y)))
                / (self.frequency * c * ((1.0 + y).powf(n) - 1.0) + self.frequency * y);
        duration
    }

    /// Finds the Modified Duration of the bond, by calling the macauly_duration() function and adjusting by the factor.
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
