

struct SimpleBond {
    face_value: f32,
    coupon: f32,
    frequency: f32,
    maturity: f32,
    yield_to_maturity: f32,
}

impl SimpleBond {
    // Need to refactor this function, to make the pv calcs more readable
    fn price(&self) -> f32{
        let pv_principal = self.face_value/((1.0 + (self.yield_to_maturity/self.frequency)).powf(self.maturity*self.frequency));

        let pv_coupon = ((self.coupon*self.face_value)/self.yield_to_maturity) * (1.0 - (1.0/((1.0 + (self.yield_to_maturity/self.frequency)).powf(self.maturity*self.frequency))));
        
        let price = pv_principal + pv_coupon;
        
        return price
    }
}

// We could set defaults as for UST and have this function build the rest of the struct
fn build_ust(coupon:f32, maturity:f32, yield_to_maturity:f32) -> SimpleBond {
    SimpleBond {
        face_value: 1000.0,
        coupon,
        frequency: 2.0,
        maturity,
        yield_to_maturity,
    }
}

fn main() {

    // reference bond for TMUBMUSD10Y on 5/2/2024
    let bond1 = SimpleBond {
        face_value: 1000.0,
        coupon: 0.04,
        frequency: 2.0,
        maturity: 10.0,
        yield_to_maturity: 0.04584,
    };

    println!("10 year UST price on 5/2/2024: ${}", bond1.price());
}
