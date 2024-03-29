#[macro_export]
macro_rules! declare_constraints {
    ($($name:ident $target:ident $(<= $($arg:ident),*)?);* $(;)?) => {
        $(declare_constraints!(@call $name $target $($($arg)*)?);)*
    };

    (@call new $constraint:ident) => {
        pub trait $constraint {}
    };

    (@call compose $target:ident $($constraint:ident)*) => {
        pub trait $target {}

        impl <T> $target for T
        where
            T: $($constraint +)*
        {}
    };
}

#[cfg(test)]
mod test {
    declare_constraints! {
        new Constraint1;
        new Constraint2;
        new Constraint3;

        compose ConstraintA <= Constraint1;
        compose ConstraintB <= Constraint1, Constraint2;
        compose ConstraintC <= Constraint1, Constraint2, Constraint3;
    }

    #[test]
    fn declare() {
        // ...
    }
}
