#![warn(clippy::pedantic)]
use std::{cell::RefCell, collections::HashMap, fmt::Display};

use crate::codebase::{Codebase, SealedState};

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
        use $crate::detector::Detector;
        pub struct $tname;

        impl $crate::detector::Detector for $tname {
            fn check(
                &self,
                $($params)*
            ) -> Option<Vec<$crate::detector::DetectorResult>> {
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
    };
    () => {};
}

pub trait CombinedDetector: Detector + DetectorReportTemplate {}

impl<T: Detector + DetectorReportTemplate> CombinedDetector for T {}

pub type CompactDetector = Box<dyn CombinedDetector>;

#[derive(Debug, Clone)]
pub struct DetectorResult {
    pub file_path: String,
    pub offset_start: u32,
    pub offset_end: u32,
    pub extra: Option<HashMap<String, String>>,
}

pub trait Detector {
    fn check(&self, codebase: &RefCell<Codebase<SealedState>>) -> Option<Vec<DetectorResult>>;
}

pub trait DetectorReportTemplate {
    fn id(&self) -> String;
    fn uid(&self) -> String;
    fn description(&self) -> String;
    fn severity(&self) -> String;
    fn tags(&self) -> Vec<String>;
    fn title_single_instance(&self) -> String;
    fn title_multiple_instance(&self) -> String;
    fn opening(&self) -> String;
    fn body_single_file_single_instance(&self) -> String;
    fn body_single_file_multiple_instance(&self) -> String;
    fn body_multiple_file_multiple_instance(&self) -> String;
    fn body_list_item_single_file(&self) -> String;
    fn body_list_item_multiple_file(&self) -> String;
    fn closing(&self) -> String;
    fn template(&self) -> String;
}

impl Display for dyn CombinedDetector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_combined_detector_display() {
        // Dummy detector implementing both traits
        struct Dummy;
        impl Detector for Dummy {
            fn check(
                &self,
                _codebase: &RefCell<Codebase<SealedState>>,
            ) -> Option<Vec<DetectorResult>> {
                Some(vec![DetectorResult {
                    file_path: "f".into(),
                    offset_start: 0,
                    offset_end: 1,
                    extra: None,
                }])
            }
        }
        impl DetectorReportTemplate for Dummy {
            fn id(&self) -> String {
                "dummy".into()
            }
            fn uid(&self) -> String {
                "uid".into()
            }
            fn description(&self) -> String {
                String::new()
            }
            fn severity(&self) -> String {
                String::new()
            }
            fn tags(&self) -> Vec<String> {
                vec![]
            }
            fn title_single_instance(&self) -> String {
                String::new()
            }
            fn title_multiple_instance(&self) -> String {
                String::new()
            }
            fn opening(&self) -> String {
                String::new()
            }
            fn body_single_file_single_instance(&self) -> String {
                String::new()
            }
            fn body_single_file_multiple_instance(&self) -> String {
                String::new()
            }
            fn body_multiple_file_multiple_instance(&self) -> String {
                String::new()
            }
            fn body_list_item_single_file(&self) -> String {
                String::new()
            }
            fn body_list_item_multiple_file(&self) -> String {
                String::new()
            }
            fn closing(&self) -> String {
                String::new()
            }
            fn template(&self) -> String {
                String::new()
            }
        }
        let det: CompactDetector = Box::new(Dummy);
        // Display should use id()
        assert_eq!(det.to_string(), "dummy");
    }
}
