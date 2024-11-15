use rmk::matrix::MatrixTrait;

use super::port::PS2Port;

pub struct PS2Matrix<const ROW: usize, const COL: usize> {
    port: PS2Port;
}

impl<const ROW: usize, const COL: usize> MatrixTrait for PS2Matrix<ROW, COL> {
    const ROW: usize = ROW;
    const COL: usize = COL;
}
