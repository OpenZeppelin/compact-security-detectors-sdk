 # Testing

 This section describes how to write and run tests for detectors.

 ## Unit Tests

 Tests are defined inline in `src/lib.rs` under `#[cfg(test)] mod tests`. See [Writing Detectors](writing-detectors.md) for examples of detector implementations and their corresponding tests.

 Example test:

 ```rust
 #[test]
 fn test_my_detector() {
     let detector = MyCustomDetector;
     let src_code = "export circuit example(): [] { ... }";
     let mut data = HashMap::new();
     data.insert("example.compact".to_string(), src_code.to_string());
     let codebase = build_codebase(&data).unwrap();
     let result = detector.check(&codebase);
     assert!(result.is_some());
     assert_eq!(result.unwrap().len(), 1);
 }
 ```

 ## Integration Tests

 For more complex scenarios, consider adding tests in a `tests/` directory at the root. Cargo automatically picks up integration tests.

 ## Running Tests

 Execute all tests:

 ```bash
 cargo test
 ```

 To run a specific test:

 ```bash
 cargo test test_my_detector
 ```

 ## Coverage

 Consider using `cargo tarpaulin` or a similar tool for coverage analysis.