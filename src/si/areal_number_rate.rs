//! Areal number rate (base unit 1 per square meter second, m⁻² · s⁻¹).

quantity! {
    /// Areal number rate (base unit 1 per square meter second, m⁻² · s⁻¹).
    quantity: ArealNumberRate; "areal number rate";
    /// Dimension of areal number rate, L⁻²T⁻¹ (base unit 1 per square meter second, m⁻² · s⁻¹).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);   
    units {
        @per_square_meter_second: prefix!(none); "m⁻²·s⁻¹",
            "per square meter second", "per square meter second";
        @per_square_centimeter_second: prefix!(none) / prefix!(centi) / prefix!(centi); "cm⁻²·s⁻¹",
            "per square centimeter second", "per square centimeter second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::areal_number_rate as anr;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::area as a;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<anr::per_square_meter_second, a::square_meter,  t::second>();
            test::<anr::per_square_centimeter_second, a::square_centimeter,  t::second>();

            fn test< ANR: anr::Conversion<V>, A: a::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&ArealNumberRate::new::<ANR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Area::new::<A>(V::one())).into());
            }
        }
    }
}
