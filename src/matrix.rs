use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::vector::{dot_product, Vector};

#[derive(Debug)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub column: usize,
}

const NUM_THREADS: usize = 4;

struct MsgInput<T> {
    row: usize,
    column: usize,
    row_data: Vector<T>,
    column_data: Vector<T>,
}

struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

struct MsgOutput<T> {
    row: usize,
    column: usize,
    result: T,
}

pub fn multiply_multithread<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + AddAssign + Add<Output = T> + Mul<Output = T> + Default + Send + 'static,
{
    let matrix_len = a.row * b.column;
    let data: Vec<T> = vec![T::default(); matrix_len];
    let mut matrix = Matrix::new(data, a.row, b.column);

    let txs = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let input = msg.input;
                    if let Ok(result) = dot_product(input.row_data, input.column_data) {
                        if let Err(e) = msg.sender.send(MsgOutput {
                            row: input.row,
                            column: input.column,
                            result,
                        }) {
                            eprintln!("Send error: {:?}", e);
                        };
                    }
                }
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut receivers = Vec::with_capacity(matrix_len);
    for row in 1..=a.row {
        for column in 1..=b.column {
            let row_data = Vector::new(a.row(row).unwrap());
            let column_data = Vector::new(b.column(column).unwrap());

            let (sender, receiver) = oneshot::channel();
            let input = MsgInput {
                row,
                column,
                row_data,
                column_data,
            };
            if let Err(e) = txs[row * column % NUM_THREADS].send(Msg { input, sender }) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(receiver);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        (*matrix.mut_value(output.row, output.column).unwrap()) = output.result;
    }

    Ok(matrix)
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + AddAssign + Add<Output = T> + Mul<Output = T> + Default,
{
    let matrix_len = a.row * b.column;
    let data: Vec<T> = vec![T::default(); matrix_len];

    let mut matrix = Matrix::new(data, a.row, b.column);

    for row in 0..a.row {
        for column in 0..b.column {
            let row_data = Vector::new(a.row(row + 1).unwrap());
            let column_data = Vector::new(b.column(column + 1).unwrap());
            (*matrix.mut_value(row + 1, column + 1).unwrap()) = dot_product(row_data, column_data)?;
        }
    }
    Ok(matrix)
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Copy + AddAssign + Add<Output = T> + Mul<Output = T> + Default + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        multiply_multithread(&self, &rhs).unwrap()
    }
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, column: usize) -> Self {
        Self {
            data: data.into(),
            row,
            column,
        }
    }

    /// return the value located at the intersection of this specific row and column.
    ///
    /// row and column start from 1
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrency::matrix::Matrix;
    ///
    /// let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);;
    /// let result = matrix.value(2, 2);
    /// assert_eq!(result, Some(&5));
    /// ```
    pub fn value(&self, row: usize, column: usize) -> Option<&T> {
        if row > self.row || column > self.column {
            None
        } else {
            let row = row - 1;
            let column = column - 1;
            Some(&self.data[row * self.column + column])
        }
    }

    pub fn mut_value(&mut self, row: usize, column: usize) -> Option<&mut T> {
        if row > self.row || column > self.column {
            None
        } else {
            let row = row - 1;
            let column = column - 1;
            Some(&mut self.data[row * self.column + column])
        }
    }

    pub fn row(&self, row: usize) -> Option<Vec<T>>
    where
        T: Copy,
    {
        if row > self.row {
            None
        } else {
            Some(self.data[(row - 1) * self.column..row * self.column].to_vec())
        }
    }

    pub fn column(&self, column: usize) -> Option<Vec<T>>
    where
        T: Copy,
    {
        if column > self.column {
            None
        } else {
            Some(
                self.data[(column - 1)..]
                    .iter()
                    .step_by(self.column)
                    .copied()
                    .collect(),
            )
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for row in 0..self.row {
            let v = (0..self.column)
                .map(|column| {
                    self.value(row + 1, column + 1)
                        .expect("row and column must be in the range")
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "{v}")?;
            if row != self.row - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 1, 2, 1, 2], 3, 2);
        let b = Matrix::new(vec![1, 2, 3, 1, 2, 3], 2, 3);
        let c = multiply(&a, &b)?;
        println!("{c}");
        assert_eq!(c.row, 3);
        assert_eq!(c.column, 3);
        Ok(())
    }

    #[test]
    fn test_matrix_value() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.value(1, 2), Some(&2));
        assert_eq!(matrix.value(2, 2), Some(&5));
        assert_eq!(matrix.value(3, 1), None);
    }

    #[test]
    fn test_matrix_display() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        println!("{}", matrix);
    }

    #[test]
    fn test_matrix_row() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.row(1), Some(vec![1, 2, 3]));
        assert_eq!(matrix.row(2), Some(vec![4, 5, 6]));
        assert_eq!(matrix.row(3), None);
    }

    #[test]
    fn test_matrix_column() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.column(1), Some(vec![1, 4]));
        assert_eq!(matrix.column(2), Some(vec![2, 5]));
        assert_eq!(matrix.column(3), Some(vec![3, 6]));
        assert_eq!(matrix.column(4), None);
    }
}
