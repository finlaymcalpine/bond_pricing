pub mod bonds;
use crate::bonds::simple_bond::SimpleBond;

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
        price: 0.0, // we have to give some float to fill out the struct. can replace with an option
    };

    let mut bond2 = SimpleBond {
        face_value: 1000.0,
        coupon: 0.04,
        frequency: 2.0,
        maturity: 10.0,
        yield_to_maturity: 0.0, // we have to give some float to fill out the struct. can replace with an option
        price: 953.5723,
    };

    let bond3 = SimpleBond {
        face_value: 1000.0,
        coupon: 0.1,
        frequency: 2.0,
        maturity: 30.0,
        yield_to_maturity: 0.1,
        price: 1000.0,
    };

    println!("10 year UST price on 5/2/2024: ${}", bond1.solve_price());
    println!(
        "10 year UST YtM on 5/2/2024: {}",
        bond2.solve_yield_to_maturity(0.05, 100, 0.00001, 0.000000000001)
    );
    println!("current yield: {}%", bond2.current_yield());
    println!("duration of 10-yr is {} years", bond2.macauly_duration());
    println!(
        "duration of 30-yr par is {} years",
        bond3.macauly_duration()
    );
    println!(
        "modified duration of 30-yr par is {} years",
        bond3.modified_duration()
    );
}
