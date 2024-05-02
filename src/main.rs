

struct SimpleBond {
    face_value: i32,
    coupon: f32,
    frequency: i32,
    maturity: f32,
    yield_to_maturity: f32,
}

impl SimpleBond {
    // Need to refactor this function, to make the pv calcs more readable
    fn price(&self) -> f32{
        let pv_principal = self.face_value as f32/((1.0 + (self.yield_to_maturity/self.frequency as f32)).powf(self.maturity*self.frequency as f32));
        let pv_coupon = ((self.coupon*self.face_value as f32)/self.yield_to_maturity) * (1.0 - (1.0/((1.0 + (self.yield_to_maturity/self.frequency as f32)).powf(self.maturity*self.frequency as f32))));
        let price = pv_principal + pv_coupon;
        price
    }
}

// We could set defaults as for UST and have this function build the rest of the struct
fn build_ust(coupon:f32, maturity:f32, yield_to_maturity:f32) -> SimpleBond {
    SimpleBond {
        face_value: 1000,
        coupon,
        frequency: 2,
        maturity,
        yield_to_maturity,
    }
}

fn main() {
    let bond1 = SimpleBond {
        face_value: 1000,
        coupon: 0.05,
        frequency: 2,
        maturity: 10.0,
        yield_to_maturity: 0.06,
    };

    println!("{}", bond1.price());
}
