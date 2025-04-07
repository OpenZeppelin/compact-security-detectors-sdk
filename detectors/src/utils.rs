#[macro_export]
macro_rules! detector {
    (
        #[type_name = $tname:ident]
        $(#[$attr:meta])*
        $vis:vis fn $name:ident $(< $($gen:tt)* >)? ( $($params:tt)* )
        $(-> $ret:ty)?
        $(where $($where:tt)*)?
        $body:block
    ) => {
        pub struct $tname;

        impl midnight_security_detectors_sdk::Detector for $tname {
            fn check(
                &self,
                $($params)*
            ) -> Option<std::collections::HashMap<String, Vec<(u32, u32)>>> {
                $body
            }
        }
    };
    () => {};
}

#[macro_export]
macro_rules! detectors {
    (
        $(
            #[type_name = $tname:ident]
            $(#[$attr:meta])*
            $vis:vis fn $name:ident $(< $($gen:tt)* >)? ( $($params:tt)* )
            $(-> $ret:ty)?
            $(where $($where:tt)*)?
            $body:block
        )*
    ) => {
        $(
            detector! {
                #[type_name = $tname]
                $(#[$attr])*
                $vis fn $name $(< $($gen)* >)? ( $($params)* )
                $(-> $ret)?
                $(where $($where)*)?
                $body
            }
        )*

        pub fn all_detectors() -> Vec<midnight_security_detectors_sdk::MidnightDetector> {
            vec![
                $(
                    Box::new($tname),
                )*
            ]
        }
    };
    () => {};
}
