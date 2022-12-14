# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2023-01-03

### Added

- `Pow` trait for types that can be exponentiated
- `Sqrt` trait for types from which a square root can be calculated

### Changed

- `magnitude`, `magnitude_squared`, `normalize` and `assign_normalize` functions on Vectors made
  more generic

## [0.8.0] - 2023-01-02

### Changed

- Make cross product output type as flexible as the other math operators

## [0.7.0] - 2023-01-02

### Added

- Dot product implementation for Vector
- Cross product implementation for Vector3 (3 dimensional only for now)

### Removed

- `DotAssign` trait (dot product returns a scalar from two Vector/Matrix inputs, so it needs to be
  stored in a third memory location)
- `CrossAssign` trait (since cross products intermingle each value with those of different indices
  in both vectors, it is impossible to store the calculations without putting them into an
  intermediate location first, which is the same as just using the Cross trait, so there's no
  need to repeat ourselves here)

## [0.6.0] - 2022-11-29

### Changed

- Vector and Matrix math operators can return values with different types than the
  left-hand-side depending on how the underlying math operators of each component type is
  implemented. Usually it will return the same as the left-hand-side though.

## [0.5.0] - 2022-11-27

### Added

- Traits for dot and cross products
- Traits for componentwise operations between types composed of multiple values of the same type
  (like Vector and Matrix)
- Implementation for componentwise operator for Vector
- Re-added `Scalar` marker type (turns out to be useful after all)
- Re-added addition and subtraction where left is a Vector and right is a Scalar
- Re-added multiplication and division where both sides are Vectors

## [0.4.0] - 2022-11-20

### Added

- Indexing operator for Matrix
- `from_array` factory functions for Vector and Matrix
- function to transpose a Matrix
- Addition, subtraction, scalar multiply, and scalar divide for Matrix

### Removed

- `Scalar` marker type

### Fixed

- Size generics for Matrix type were swapped (not consistent with the math)

## [0.3.0] - 2022-11-20

### Added

- Initial implementation of `Matrix` (just the struct for now)
- Indexing operator for Vector
- Scalar operators for Vector (such as adding a vector and a scalar, etc.)
- Magnitude squared function, for when you want to skip the expensive
  square root calculation
- Functions to normalize a Vector

### Changed

- rename `*_vector_op` to `*_componentwise_op` to be more consistent with math terminology

### Removed

- Add and subtract where left is a Vector and right is a scalar
- Multiply and divide where both sides are vectors

## [0.2.0] - 2022-11-20

### Changed

- Vector operators are now more flexible and can handle type differences better
- Function to calculate the magnitude of a Vector

### Fixed

- Componentwise operations were skipping the last element in a vector

## [0.1.0] - 2022-11-19

### Added

- Initial implementation of the `Vector` type

