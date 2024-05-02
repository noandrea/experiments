use std::u32;

use pallet_referenda::Curve;

// Perbill is a fixed point representation of a number in the range 0, 1.
use sp_runtime::Perbill;

const fn percent(x: i32) -> sp_runtime::FixedI64 {
    sp_runtime::FixedI64::from_rational(x as u128, 100)
}
const fn permill(x: i32) -> sp_runtime::FixedI64 {
    sp_runtime::FixedI64::from_rational(x as u128, 1000)
}

// Entrypoint of the application, where the execution starts
fn main() {
    // define the list of curves to be printed
    // each curve is a tuple with the following elements:
    // - name of the curve
    // - decision period in days
    // - approval curve
    // - support curve
    //
    // There are two types of curves:
    // - Reciprocal: the curve is a reciprocal function that starts at 100% and goes to 0%.
    // - Linear: the curve is a linear function that starts at 0% and goes to 100%.
    // see the implementation of the Curve struct for more details here:
    // https://github.com/paritytech/polkadot-sdk/blob/9ede4152ef0d539019875e6aff97dbe0744a4053/substrate/frame/referenda/src/types.rs#L385
    // The reciprocal curves are defined by the following parameters:
    // - delay:
    // - period: the duration of the curve in days
    // - level: the threshold at which the curve starts to change
    // - floor : the threshold at which the
    // - ceil : the threshold at which the curve reaches the period value
    let curves = vec![
        (
            "MB ROOT",
            14,
            Curve::make_reciprocal(4, 14, percent(80), percent(50), percent(100)),
            Curve::make_linear(14, 14, permill(5), percent(25)),
        ),
        (
            "MB Whitelist",
            14,
            Curve::make_reciprocal(1, 14, percent(96), percent(50), percent(100)),
            Curve::make_reciprocal(1, 14 * 24, percent(1), percent(0), percent(2)),
        ),
        (
            "MB GA",
            14,
            Curve::make_reciprocal(4, 14, percent(80), percent(50), percent(100)),
            Curve::make_reciprocal(7, 14, percent(10), percent(0), percent(50)),
        ),
        (
            "MB Referendum canceller",
            14,
            Curve::make_reciprocal(1, 14, percent(96), percent(50), percent(100)),
            Curve::make_reciprocal(1, 14, percent(1), percent(0), percent(10)),
        ),
        (
            "MB Referendum killer",
            14,
            Curve::make_reciprocal(1, 14, percent(96), percent(50), percent(100)),
            Curve::make_reciprocal(1, 14, percent(1), percent(0), percent(10)),
        ),
        (
            "MB FGA",
            14,
            Curve::make_reciprocal(4, 14, percent(80), percent(50), percent(100)),
            Curve::make_reciprocal(5, 14, percent(1), percent(0), percent(50)),
        ),
    ];

    // loop over the curves
    for (name, decision_period, approval, support) in curves {
        // print the curve values
        print_curve(name, decision_period, &approval, &support);
    }

    //
}

// print the curve values
// the output of the print is a table that is formatted to be copy pasted
// into a spreadsheet that it is used to plot the curve
fn print_curve(name: &str, decision_period: u32, approval: &Curve, support: &Curve) {
    println!("{}\n", name);
    println!("Hours\tApproval\tSupport");

    let hours = decision_period * 24;
    let perbills = vec![
        // hours
        (0, hours),
        (1, hours),
        (2, hours),
        (3, hours),
        (6, hours),
        (12, hours),
        // days
        ((decision_period / 12) * 24, hours),
        ((decision_period / 8) * 24, hours),
        ((decision_period / 4) * 24, hours),
        ((decision_period / 2) * 24, hours),
        ((decision_period * 3 / 4) * 24, hours),
        (decision_period * 24, hours),
    ];

    let mut last_h = u32::MAX;
    for (p, q) in perbills {
        if p == last_h {
            continue;
        }
        let b = Perbill::from_rational(p, q);
        let a = approval.threshold(b);
        let s = support.threshold(b);
        println!("{:?},{:?},{:?}", p, s, a);

        last_h = p;
    }

    // approval.info(14, "approval");
    // support.info(14, "support");
}
