//! Volumetric number density (base unit 1 per cubic meter, m⁻³).

quantity! {
    /// Volumetric number density (base unit 1 per cubic meter, m⁻³).
    quantity: VolumetricNumberDensity; "volumetric number density";
    /// Dimension of volumetric number density, L⁻³ (base unit 1 per cubic meter, m⁻³).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @per_cubic_kilometer: prefix!(none) / prefix!(kilo) / prefix!(kilo) / prefix!(kilo); "km⁻³",
            "per cubic kilometer", "per cubic kilometer";
        @per_cubic_meter: prefix!(none); "m⁻³",
            "per cubic meter", "per cubic meter";
        @per_cubic_decimeter: prefix!(none) / prefix!(deci) / prefix!(deci) / prefix!(deci); "dm⁻³",
            "per cubic decimeter", "per cubic decimeter";
        @per_cubic_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "cm⁻³",
            "per cubic centimeter", "per cubic centimeter";
        @per_cubic_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli) / prefix!(milli); "mm⁻³",
            "per cubic millimeter", "per cubic millimeter";            
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::volume as v;
        use crate::si::volumetric_number_density as n;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricNumberDensity<V> = (V::one()
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<n::per_cubic_kilometer, v::cubic_kilometer>();
            test::<n::per_cubic_meter, v::cubic_meter>();
            test::<n::per_cubic_decimeter, v::cubic_decimeter>();
            test::<n::per_cubic_centimeter, v::cubic_centimeter>();
            test::<n::per_cubic_millimeter, v::cubic_millimeter>();

            fn test<N: n::Conversion<V>, U: v::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricNumberDensity::new::<N>(V::one()),
                    &(V::one() / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
