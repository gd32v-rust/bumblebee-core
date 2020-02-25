//! Bumblebee core defined RISC-V CSR's

/// mcountinhibit register
pub mod mcountinhibit {
    read_csr_as_usize_rv32!(0x320, __read_mcountinhibit);
    write_csr_as_usize_rv32!(0x320, __write_mcountinhibit);
}
/// mnvec register
pub mod mnvec {
    read_csr_as_usize_rv32!(0x7c3, __read_mnvec);
}
/// msubm register
pub mod msubm {
    read_csr_as_usize_rv32!(0x7c4, __read_msubm);
    write_csr_as_usize_rv32!(0x7c4, __write_msubm);
}
/// mmisc_ctl register
pub mod mmisc_ctl {
    read_csr_as_usize_rv32!(0x7d0, __read_mmisc_ctl);
    write_csr_as_usize_rv32!(0x7d0, __write_mmisc_ctl);
}
/// msavestatus register
pub mod msavestatus {
    read_csr_as_usize_rv32!(0x7d6, __read_msavestatus);
    write_csr_as_usize_rv32!(0x7d6, __write_msavestatus);
}
/// msaveepc1 register
pub mod msaveepc1 {
    read_csr_as_usize_rv32!(0x7d7, __read_msaveepc1);
    write_csr_as_usize_rv32!(0x7d7, __write_msaveepc1);
}
/// msavecause1 register
pub mod msavecause1 {
    read_csr_as_usize_rv32!(0x7d8, __read_msavecause1);
    write_csr_as_usize_rv32!(0x7d8, __write_msavecause1);
}
/// msaveepc2 register
pub mod msaveepc2 {
    read_csr_as_usize_rv32!(0x7d9, __read_msaveepc2);
    write_csr_as_usize_rv32!(0x7d9, __write_msaveepc2);
}
/// msavecause2 register
pub mod msavecause2 {
    read_csr_as_usize_rv32!(0x7da, __read_msavecause2);
    write_csr_as_usize_rv32!(0x7da, __write_msavecause2);
}
/// pushmsubm register
pub mod pushmsubm {
    read_csr_as_usize_rv32!(0x7eb, __read_pushmsubm);
    write_csr_as_usize_rv32!(0x7eb, __write_pushmsubm);
}
/// mtvt2 register
pub mod mtvt2 {
    read_csr_as_usize_rv32!(0x7ec, __read_mtvt2);
    write_csr_as_usize_rv32!(0x7ec, __write_mtvt2);
}
/// jalmnxti register
pub mod jalmnxti {
    read_csr_as_usize_rv32!(0x7ed, __read_jalmnxti);
    write_csr_as_usize_rv32!(0x7ed, __write_jalmnxti);
}
/// pushmcause register
pub mod pushmcause {
    read_csr_as_usize_rv32!(0x7ee, __read_pushmcause);
    write_csr_as_usize_rv32!(0x7ee, __write_pushmcause);
}
/// pushmepc register
pub mod pushmepc {
    read_csr_as_usize_rv32!(0x7ef, __read_pushmepc);
    write_csr_as_usize_rv32!(0x7ef, __write_pushmepc);
}
/// sleepvalue register
pub mod sleepvalue {
    read_csr_as_usize_rv32!(0x811, __read_sleepvalue);
    write_csr_as_usize_rv32!(0x811, __write_sleepvalue);
}
/// txevt register
pub mod txevt {
    read_csr_as_usize_rv32!(0x812, __read_txevt);
    write_csr_as_usize_rv32!(0x812, __write_txevt);
}
/// wfe register
pub mod wfe {
    read_csr_as_usize_rv32!(0x810, __read_wfe);
    write_csr_as_usize_rv32!(0x810, __write_wfe);
}
