
//! Magnetic field strength (H field) (base unit ampere per meter, A · m⁻¹). 

quantity! {
    /// Magnetic field strength (H field) (base unit ampere per meter, A · m⁻¹).
    quantity: MagneticFieldStrength; "magnetic field strength (H field)";
    /// Dimension of magnetic field strength (H field), IL⁻¹ (base unit ampere per meter, A · m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @ampere_per_meter: prefix!(none); "A·m⁻¹", "ampere per meter", "amperes per meter";
        @ampere_per_centimeter: prefix!(none) / prefix!(centi); "A·cm⁻¹", "ampere per centimeter", "amperes per centimeter";
        @ampere_per_millimeter: prefix!(none) / prefix!(milli); "A·mm⁻¹", "ampere per millimeter", "amperes per millimeter";
        @ampere_per_micrometer: prefix!(none) / prefix!(micro); "A·μm⁻¹", "ampere per micrometer", "amperes per micrometer";

        @oersted: 79.577_471_545_947_67; "Oe", "oersted", "oersteds"; // 1 oersted = 1000/(4π)  A/m
    }
}


#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::electric_current as i;
        use crate::si::magnetic_field_strength as h;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MagneticFieldStrength<V> = ElectricCurrent::new::<i::ampere>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<i::ampere, l::meter, h::ampere_per_meter>();
            test::<i::ampere, l::centimeter, h::ampere_per_centimeter>();
            test::<i::ampere, l::millimeter, h::ampere_per_millimeter>();
            test::<i::ampere, l::micrometer, h::ampere_per_micrometer>();

            fn test<I: i::Conversion<V>, L: l::Conversion<V>, H: h::Conversion<V>>() {
                Test::assert_approx_eq(&MagneticFieldStrength::new::<H>(V::one()),
                    &(ElectricCurrent::new::<I>(V::one()) / Length::new::<L>(V::one())));
            }
        }
    }
}