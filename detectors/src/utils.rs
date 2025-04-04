#[macro_export]
macro_rules! detector {
    ($name:ident, $param1:expr, $param2:expr, $param3:expr => $check_fn:ident) => {
        pub struct $name;

        impl midnight_security_detectors_sdk::Detector for $name {
            fn name(&self) -> String {
                stringify!($name).to_string()
            }

            fn description(&self) -> String {
                $param1.to_string()
            }

            fn severity(&self) -> String {
                $param2.to_string()
            }

            fn tags(&self) -> Vec<String> {
                vec!["security".to_string(), $param3.to_string()]
            }

            fn check(
                &self,
                codebase: &std::cell::RefCell<
                    midnight_security_detectors_sdk::codebase::Codebase<
                        midnight_security_detectors_sdk::codebase::SealedState,
                    >,
                >,
            ) -> Option<std::collections::HashMap<String, Vec<(u32, u32)>>> {
                $check_fn(codebase)
            }
        }
    };
    () => {};
}

#[macro_export]
macro_rules! detectors {
    (
        $($name:ident, $param1:expr, $param2:expr, $param3:expr => $check_fn:ident),* $(,)*
    ) => {
        $(
            detector!($name, $param1, $param2, $param3 => $check_fn);
        )*

        pub fn all_detectors() -> Vec<Box<dyn midnight_security_detectors_sdk::Detector>> {
            vec![
                $(
                    Box::new($name),
                )*
            ]
        }
    };
    ($($name:ident, $param1:expr, $param2:expr, $param3:expr => $check_fn:ident),* $(,)*) => {

    };
}
