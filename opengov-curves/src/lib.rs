use pallet_referenda::Curve;
use sp_runtime::Perbill;

pub extern "C" fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[repr(C)]
pub struct Point {
    x: u32,
    y: u32,
}

const fn percent(x: i32) -> sp_runtime::FixedI64 {
    sp_runtime::FixedI64::from_rational(x as u128, 100)
}

pub extern "C" fn make_curve(
    decision_period: u32,
    delay: u64,
    period: u64,
    level: i32,
    floor: i32,
    ceil: i32,
) -> Vec<Point> {
    let mut points = Vec::new();
    let hours = decision_period * 24;
    let perbills: Vec<(u32, u32)> = vec![
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

    let curve = Curve::make_reciprocal(
        delay.into(),
        period.into(),
        percent(level),
        percent(floor),
        percent(ceil),
    );

    let mut last_h = u32::MAX;
    for (p, q) in perbills {
        if p == last_h {
            continue;
        }
        let b = Perbill::from_rational(p, q);
        let a = curve.threshold(b);
        let point = Point {
            x: p,
            y: a.deconstruct(),
        };
        points.push(point);
        last_h = p;
    }
    points
}
