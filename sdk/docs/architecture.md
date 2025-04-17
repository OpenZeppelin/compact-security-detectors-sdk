<!-- docs/architecture.md -->
[← Back to Index](index.md)

# Architecture

The SDK consists of several core layers:

1. AST Construction (`ast`):
   - Parses source code using Tree-sitter.
   - Builds an in-memory AST (`Program`, `NodeType`, etc.).
2. Codebase (`codebase`):
   - Manages a collection of source files and their ASTs.
   - Builds symbol tables and resolves imports and function calls.
3. Passes (`passes`):
   - Implements algorithms to build and merge symbol tables.
4. Storage (`storage`):
   - Stores all AST nodes in a flat structure with parent-child links.
5. Detector Framework:
   - Traits `Detector` and `DetectorReportTemplate` define the interface for writing detectors.
   - Combines check logic with reporting.

```text
Source files ➔ AST Builder ➔ NodesStorage ➔ Symbol Tables ➔ Codebase.Seal ➔ Detector.run
```

[Previous: Getting Started](getting_started.md)  
[Next: Modules](modules.md)