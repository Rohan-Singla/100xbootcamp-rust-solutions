/*
  Problem 34: Matrix Transpose

  Write a function that takes a Vec<Vec<i32>> representing a matrix and returns its transpose.
  The transpose of a matrix swaps rows and columns. Assume the input is a valid rectangular
  matrix (all rows have the same length). Return an empty vec for empty input.

  Run the tests for this problem with:
    cargo test --test matrix_transpose_test
*/

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  if matrix.is_empty() {
      return vec![];
  }

  let rows = matrix.len();
  let cols = matrix[0].len();


  // Create 3 copies of [0,0].
  
  let mut result = vec![vec![0; rows]; cols];

  for i in 0..rows {
      for j in 0..cols {
          result[j][i] = matrix[i][j];
      }
  }

  result
}