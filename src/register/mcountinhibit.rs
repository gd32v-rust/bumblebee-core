//! mcountinhibit register
//! 
use bit_field::BitField;

/// Controls count of mcycle and minstret.
#[derive(Clone, Copy, Debug)]
pub struct McountInhibit {
    bits: usize,
}

impl McountInhibit {
    /// Counting process of mcycle is turned off when CY is 1
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }
    /// Counting process of minstret is turned off when IR is 1.
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }
}

read_csr_as!(McountInhibit, 0x320, __read_mcountinhibit);
set!(0x320, __set_mcountinhibit);
clear!(0x320, __clear_mcountinhibit);

set_clear_csr!(
    /// Counting process of mcycle is turned off when CY is 1
    , set_cy, clear_cy, 1 << 0);
set_clear_csr!(
    /// Counting process of minstret is turned off when IR is 1.
    , set_ir, clear_ir, 1 << 2);
