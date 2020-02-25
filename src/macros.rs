// Ref: crate `riscv`

macro_rules! read_csr_rv32 {
    ($csr_number:expr, $asm_fn: ident) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                #[cfg(all(riscv32, feature = "inline-asm"))]
                () => {
                    let r: usize;
                    asm!("csrrs $0, $1, x0" : "=r"(r) : "i"($csr_number) :: "volatile");
                    r
                }

                #[cfg(all(riscv32, not(feature = "inline-asm")))]
                () => {
                    extern "C" {
                        fn $asm_fn() -> usize;
                    }

                    $asm_fn()
                }

                #[cfg(not(riscv32))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! read_csr_as_usize_rv32 {
    ($csr_number:expr, $asm_fn: ident) => {
        read_csr_rv32!($csr_number, $asm_fn);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe{ _read() }
        }
    };
}

macro_rules! write_csr_rv32 {
    ($csr_number:expr, $asm_fn: ident) => {
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            match () {
                #[cfg(all(riscv32, feature = "inline-asm"))]
                () => asm!("csrrw x0, $1, $0" :: "r"(bits), "i"($csr_number) :: "volatile"),

                #[cfg(all(riscv32, not(feature = "inline-asm")))]
                () => {
                    extern "C" {
                        fn $asm_fn(bits: usize);
                    }

                    $asm_fn(bits);
                }

                #[cfg(not(riscv32))]
                () => unimplemented!(),
            }
        }
    };
}

macro_rules! write_csr_as_usize_rv32 {
    ($csr_number:expr, $asm_fn: ident) => {
        write_csr_rv32!($csr_number, $asm_fn);

        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
            unsafe{ _write(bits) }
        }
    };
}
