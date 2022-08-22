//! Linear number rate (base unit 1 per meter second, m⁻¹ · s⁻¹).

quantity! {
    /// Linear number rate (base unit 1 per meter second, m⁻¹ · s⁻¹).
    quantity: LinearNumberRate; "linear number rate";
    /// Dimension of linear number rate, L⁻¹T⁻¹ (base unit 1 per meter second, m⁻¹ · s⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @per_meter_second: prefix!(none); "m⁻¹·s⁻¹",
            "per meter second", "per meter second";
        @per_centimeter_second: prefix!(none) / prefix!(centi); "cm⁻¹·s⁻¹",
            "per centimeter second", "per centimeter second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::linear_number_rate as lnr;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: LinearNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<lnr::per_meter_second, l::meter,  t::second>();
            test::<lnr::per_centimeter_second, l::centimeter,  t::second>();

            fn test<LNR: lnr::Conversion<V>, L: l::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&LinearNumberRate::new::<LNR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Length::new::<L>(V::one())).into());
            }
        }
    }
}
