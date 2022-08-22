//! Volumetric number rate (base unit 1 per cubic meter second, m⁻³ · s⁻¹).

quantity! {
    /// Volumetric number rate (base unit 1 per cubic meter second, m⁻³ · s⁻¹).
    quantity: VolumetricNumberRate; "volumetric number rate";
    /// Dimension of volumetric number rate, L⁻³T⁻¹ (base unit 1 per cubic meter second, m⁻³ · s⁻¹).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);

    units {
        @per_cubic_meter_second: prefix!(none); "m⁻³·s⁻¹",
            "per cubic meter second", "per cubic meter second";
        @per_cubic_centimeter_second: prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "cm⁻³·s⁻¹",
            "per cubic centimeter second", "per cubic centimeter second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::volumetric_number_rate as vnr;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::volume as vol;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Volume::new::<vol::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::< vnr::per_cubic_meter_second, vol::cubic_meter,  t::second>();
            test::< vnr::per_cubic_centimeter_second, vol::cubic_centimeter,  t::second>();

            fn test< VNR: vnr::Conversion<V>, VOL: vol::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricNumberRate::new::<VNR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Volume::new::<VOL>(V::one())).into());
            }
        }
    }
}
