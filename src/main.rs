struct SimpleBond {
    face_value: f32,
    coupon: f32,
    frequency: f32,
    maturity: f32,
    yield_to_maturity: f32,
    price: f32,
}

impl SimpleBond {
    // Need to refactor this function, to make the pv calcs more readable
    fn price(&mut self) -> f32 {
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
    fn yield_to_maturity(&mut self, mut x0: f32, iter: i32, tolerance: f32, epsilon: f32) -> f32 {
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
}

// We could set defaults as for UST and have this function build the rest of the struct
fn build_ust(coupon: f32, maturity: f32, yield_to_maturity: f32, price: f32) -> SimpleBond {
    SimpleBond {
        face_value: 1000.0,
        coupon,
        frequency: 2.0,
        maturity,
        yield_to_maturity,
        price,
    }
}

fn main() {
    // reference bond for TMUBMUSD10Y on 5/2/2024
    let mut bond1 = SimpleBond {
        face_value: 1000.0,
        coupon: 0.04,
        frequency: 2.0,
        maturity: 10.0,
        yield_to_maturity: 0.04584,
        price: 0.0,
    };

    let mut bond2 = SimpleBond {
        face_value: 1000.0,
        coupon: 0.04,
        frequency: 2.0,
        maturity: 10.0,
        yield_to_maturity: 0.0, // we have to give some float to fill out the struct
        //price: 951.25,
        price: 953.5723, 
    };

    println!("10 year UST price on 5/2/2024: ${}", bond1.price());
    println!(
        "10 year UST YtM on 5/2/2024: {}",
        bond2.yield_to_maturity(0.05, 100, 0.00001, 0.000000000001)
    );
}
