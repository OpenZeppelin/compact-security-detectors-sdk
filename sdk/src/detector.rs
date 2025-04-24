#![warn(clippy::pedantic)]
//! Detector module
//! This module provides the Detector trait and macros for implementing detectors.
//!
//! Public members:
//! - `detector!` macro for defining a detector. It automatically creates the structure for the provided `type_name` in the arrtibute and implements `Detector` trait. It can be applied to a single function with `type_name` attribute and follows `check` function signature.
//! - `detectors!` macro for defining multiple detectors at once. It can be applied to a list of functions with `type_name` attribute and follows `check` function signature.
//! - `Detector` trait for implementing a detector. It has a single method `check` that takes a `Codebase` and returns an optional vector of `DetectorResult`.
//! - `DetectorResult` struct for representing the result of a detector. It contains the file path, start and end offsets, and an optional map of extra information. Extra information is used to store a map of symbol replacements in the detector template. \
//!   For example, if the detector template contains a symbol `$NAME`, the extra information can be used to replace it with the actual name.
//! - `DetectorReportTemplate` trait for implementing a detector report template. It has methods for generating the report title, body, and closing.
//! - `CombinedDetector` a union trait to force the implementor to implement both `Detector` and `DetectorReportTemplate` traits.
//! - `CompactDetector` a boxed version of `CombinedDetector`.
//! - `DetectorOpaque` a struct that is used to wrap a raw pointer to a detector. It is used to operate with detectors using C API.
use std::{collections::HashMap, fmt::Display};

use crate::codebase::{Codebase, SealedState};

/// Detector macro
/// This macro is used to define a detector. It accepts a function (signature and body) with a `type_name` attribute.
/// The function signature must follow the `check` function signature from the `Detector` trait.
/// It automatically creates the structure for the provided `type_name` in the attribute and implements the `Detector` trait.
/// The `DetectorReportTemplate` trait should be implemented to satisfy the `ComdinedDetector` contract.
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

#[repr(C)]
pub struct DetectorOpaque {
    _private: [u8; 0],
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
    fn check(&self, codebase: &Codebase<SealedState>) -> Option<Vec<DetectorResult>>;
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

    #[test]
    fn test_combined_detector_display() {
        // Dummy detector implementing both traits
        struct Dummy;
        impl Detector for Dummy {
            fn check(&self, _codebase: &Codebase<SealedState>) -> Option<Vec<DetectorResult>> {
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
