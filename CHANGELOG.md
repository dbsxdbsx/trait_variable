# Changelog

All notable changes to this project will be documented in this file.

## 0.7.0 - unreleased
* Add feature/test for trait constant and associated type as trait field
* Add feature/test for original trait parents(bound) of the target trait
* Add feature/test for trait with more complex corner cases, like generics with bounds, where clauses along with explicit parent trait(s), etc.
* Refine and add 3 integrated tests---`basic.rs`, `practical.rs`, and `complex.rs`---to cover all features
* Checking tests under Rust version 1.77.0 as default version
* Update crate `Syn` from V1.0 to V2.0
* Miscellaneous fixes and improvements
* Refine `README.md`

## 0.6.0- March 9, 2024
* Refine tests
* Add section "Limitations" to README.md
* Add and fix feature/test for tuple as trait field
* Add feature/test for HashSet as trait field
* Add feature/test for Customized Struct as trait field
* Add feature/test for Enum as trait field
* Add feature/test for Dict(BtreeMap, similar to HashMap, but in order) as trait field

## 0.5.0 - March 7, 2024

* Refactor
* Add feature/test for Vec as trait field
* Add feature/test for String as trait field

## 0.1.0~0.4.0

* Initial release, under development