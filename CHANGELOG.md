# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.20.2] - 2023-05-27

### Added

- More tests for `MatrixAreaIterator`

### Fixed

- `MatrixAreaIterator` was supposed to be inclusive of both the start and end coordinates, but
  was excluding both the ending row and column instead
- `MatrixAreaIterator` would sometimes panic when iterating in reverse due to subtracting from 0
  for an unsigned integer
- `MatrixAreaIterator::new` would sometimes panic for areas of certain dimensions due to using
  the wrong dimension in a size calculation

## [0.20.1] - 2023-05-27

### Added

- Tests for `MatrixAreaIterator`
- Implemented `FusedIterator` and `ExactSizeIterator` for `MatrixAreaIterator`

### Fixed

- rewrote `MatrixAreaIterator` to simplify, to fix iterating 1 element past the last row in the area

## [0.20.0] - 2023-05-27

### Added

- `into_iter_for` and `as_iter_for` methods on `Vector` and `Matrix` (to be able to choose custom
  indices to
  iterate)

### Changed

- `VectorRefIterator` and `MatrixRefIterator` renamed to `VectorReferenceIterator` and
  `MatrixReferenceIterator` respectively.
- `VectorIterator`, `VectorReferenceIterator`, `MatrixIterator`, and `MatrixReferenceIterator`
  now use generic iterators for indices.

### Removed

- `VectorIndexIterator` trait (this was part of my original approach to index iterators that got
  scrapped in favor of using the built-in iterator traits)

## [0.19.0] - 2023-05-22

### Added

- Generic parameters on `Vector` iterators to allow customization of the order/set iterated

## [0.18.0] - 2023-05-21

### Changed

- `row` and `column` fields of `MatrixCoordinate` are now public
- `VectorIterator`, `VectorRefIterator`, `MatrixIterator`, and `MatrixRefIterator` now implement
  `FusedIterator` for optimization purposes

## [0.17.0] - 2023-05-21

### Added

- `VectorIterator` and `VectorRefIterator` for iterating over `Vector`
- Implemented `IntoIterator` for `Vector` and references to `Vector`
- `MatrixIterator` and `MatrixRefIterator` for iterating (along rows) over `Matrix`
- Implemented `IntoIterator` for `Matrix` and references to `Matrix`
- `MatrixCoordinate` type

### Changed

- `Matrix` should now be indexed using `MatrixCoordinate` instead of a tuple (this makes it
  clearer which is the row vs column)

## [0.16.0] - 2023-04-22

### Added

- Negation operator for `Matrix` (`Neg` trait)
- Unit tests for negation of `Vector` and `Matrix`

## [0.15.0] - 2023-04-15

### Added

- Unit tests for vector math
- Negation operator for `Vector` (`Neg` trait)

## [0.14.0] - 2023-04-11

### Added

- Unit tests for Matrix-Matrix and Matrix-Scalar multiplication

### Fixed

- Matrix-Matrix multiplication was incorrect due to reversing rows and columns in the operation

## [0.13.0] - 2023-04-02

### Added

- Loads of documentation for basically everything
- `#![must_use]` annotations on any methods that return a value and don't mutate an input

### Changed

- `Matrix` now implements `Eq` and `PartialEq`
- Methods that need to initialize a `0`, `1`,
  or `-1` value for a generic type now use `From<i8>` instead
  of `Default` to do so
- Linter now denies a lot more things

## [0.12.0] - 2023-02-26

### Added

- Functions to create matrices and vectors from a single value
- Functions to create zero and identity matrices
- Function to create zero vectors
- `Default` implemented for `Vector` and `Matrix`
- `Scalar` marker implemented for `isize` and `usize`

### Changed

- `Matrix::from_array` and `Vector::from_array` are now const

## [0.11.0] - 2023-01-17

### Added

- `Matrix` to `Vector` multiplication
- `Vector::from_fn` like `Matrix::from_fn`, to create a vector component-by-component via a function

## [0.10.0] - 2023-01-15

### Added

- `Matrix` multiplication
- `Matrix::from_fn` to create a matrix component-by-component via a function

### Changed

- `ComponentwiseOp` trait renamed to `Componentwise`, `componentwise_op` function to `componentwise`
- `AssignComponentwiseOp` trait renamed to `AssignComponentwise`, `assign_componentwise_op`
  function to `assign_componentwise_op`
- Componentwise operator can use a closure with captured variables

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

