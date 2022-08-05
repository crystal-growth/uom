//! Electric current density (base unit ampere per square meter, A · m⁻²).

quantity! {
    /// Electric current density (base unit ampere per square meter, A · m⁻²).
    quantity: ElectricCurrentDensity; "electric current density";
    /// Dimension of electric current density, IL⁻² (base unit ampere per square meter, A · m⁻²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @ampere_per_square_meter: prefix!(none); "A·m⁻²", "ampere per square meter", "amperes per square meter";
        @ampere_per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "A·cm⁻²", "ampere per square centimeter", "amperes per square centimeter";
        @ampere_per_square_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli); "A·mm⁻²", "ampere per square millimeter", "amperes per square millimeter";
    }
}


#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::electric_current as i;
        use crate::si::electric_current_density as ecd;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricCurrentDensity<V> = ElectricCurrent::new::<i::ampere>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {

            test::<ecd::ampere_per_square_meter, i::ampere, a::square_meter>();
            test::<ecd::ampere_per_square_centimeter, i::ampere, a::square_centimeter>();
            test::<ecd::ampere_per_square_millimeter, i::ampere, a::square_millimeter>();

            fn test<ECD: ecd::Conversion<V>, I: i::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricCurrentDensity::new::<ECD>(V::one()),
                    &(ElectricCurrent::new::<I>(V::one()) / Area::new::<A>(V::one())));
            }
        }
    }
}