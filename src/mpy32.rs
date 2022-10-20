#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 16-bit operand one multiply"]
    pub mpy: MPY,
    #[doc = "0x02 - 16-bit operand one signed multiply"]
    pub mpys: MPYS,
    #[doc = "0x04 - 16-bit operand one multiply accumulate"]
    pub mac: MAC,
    #[doc = "0x06 - 16-bit operand one signed multiply accumulate"]
    pub macs: MACS,
    #[doc = "0x08 - 16-bit operand two"]
    pub op2: OP2,
    #[doc = "0x0a - 16x16-bit result low word"]
    pub reslo: RESLO,
    #[doc = "0x0c - 16x16-bit result high word"]
    pub reshi: RESHI,
    #[doc = "0x0e - 16x16-bit sum extension register"]
    pub sumext: SUMEXT,
    #[doc = "0x10 - 32-bit operand 1 multiply low word"]
    pub mpy32l: MPY32L,
    #[doc = "0x12 - 32-bit operand 1 multiply high word"]
    pub mpy32h: MPY32H,
    #[doc = "0x14 - 32-bit operand 1 signed multiply low word"]
    pub mpys32l: MPYS32L,
    #[doc = "0x16 - 32-bit operand 1 signed multiply high word"]
    pub mpys32h: MPYS32H,
    #[doc = "0x18 - 32-bit operand 1 multiply accumulate low word"]
    pub mac32l: MAC32L,
    #[doc = "0x1a - 32-bit operand 1 multiply accumulate high word"]
    pub mac32h: MAC32H,
    #[doc = "0x1c - 32-bit operand 1 signed multiply accumulate low word"]
    pub macs32l: MACS32L,
    #[doc = "0x1e - 32-bit operand 1 signed multiply accumulate high word"]
    pub macs32h: MACS32H,
    #[doc = "0x20 - 32-bit operand 2 low word"]
    pub op2l: OP2L,
    #[doc = "0x22 - 32-bit operand 2 high word"]
    pub op2h: OP2H,
    #[doc = "0x24 - 32x32-bit result 0 least significant word"]
    pub res0: RES0,
    #[doc = "0x26 - 32x32-bit result 1"]
    pub res1: RES1,
    #[doc = "0x28 - 32x32-bit result 2"]
    pub res2: RES2,
    #[doc = "0x2a - 32x32-bit result 3 most significant word"]
    pub res3: RES3,
    #[doc = "0x2c - MPY32 control register 0"]
    pub mpy32ctl0: MPY32CTL0,
}
#[doc = "MPY (rw) register accessor: an alias for `Reg<MPY_SPEC>`"]
pub type MPY = crate::Reg<mpy::MPY_SPEC>;
#[doc = "16-bit operand one multiply"]
pub mod mpy;
#[doc = "MPYS (rw) register accessor: an alias for `Reg<MPYS_SPEC>`"]
pub type MPYS = crate::Reg<mpys::MPYS_SPEC>;
#[doc = "16-bit operand one signed multiply"]
pub mod mpys;
#[doc = "MAC (rw) register accessor: an alias for `Reg<MAC_SPEC>`"]
pub type MAC = crate::Reg<mac::MAC_SPEC>;
#[doc = "16-bit operand one multiply accumulate"]
pub mod mac;
#[doc = "MACS (rw) register accessor: an alias for `Reg<MACS_SPEC>`"]
pub type MACS = crate::Reg<macs::MACS_SPEC>;
#[doc = "16-bit operand one signed multiply accumulate"]
pub mod macs;
#[doc = "OP2 (rw) register accessor: an alias for `Reg<OP2_SPEC>`"]
pub type OP2 = crate::Reg<op2::OP2_SPEC>;
#[doc = "16-bit operand two"]
pub mod op2;
#[doc = "RESLO (rw) register accessor: an alias for `Reg<RESLO_SPEC>`"]
pub type RESLO = crate::Reg<reslo::RESLO_SPEC>;
#[doc = "16x16-bit result low word"]
pub mod reslo;
#[doc = "RESHI (rw) register accessor: an alias for `Reg<RESHI_SPEC>`"]
pub type RESHI = crate::Reg<reshi::RESHI_SPEC>;
#[doc = "16x16-bit result high word"]
pub mod reshi;
#[doc = "SUMEXT (rw) register accessor: an alias for `Reg<SUMEXT_SPEC>`"]
pub type SUMEXT = crate::Reg<sumext::SUMEXT_SPEC>;
#[doc = "16x16-bit sum extension register"]
pub mod sumext;
#[doc = "MPY32L (rw) register accessor: an alias for `Reg<MPY32L_SPEC>`"]
pub type MPY32L = crate::Reg<mpy32l::MPY32L_SPEC>;
#[doc = "32-bit operand 1 multiply low word"]
pub mod mpy32l;
#[doc = "MPY32H (rw) register accessor: an alias for `Reg<MPY32H_SPEC>`"]
pub type MPY32H = crate::Reg<mpy32h::MPY32H_SPEC>;
#[doc = "32-bit operand 1 multiply high word"]
pub mod mpy32h;
#[doc = "MPYS32L (rw) register accessor: an alias for `Reg<MPYS32L_SPEC>`"]
pub type MPYS32L = crate::Reg<mpys32l::MPYS32L_SPEC>;
#[doc = "32-bit operand 1 signed multiply low word"]
pub mod mpys32l;
#[doc = "MPYS32H (rw) register accessor: an alias for `Reg<MPYS32H_SPEC>`"]
pub type MPYS32H = crate::Reg<mpys32h::MPYS32H_SPEC>;
#[doc = "32-bit operand 1 signed multiply high word"]
pub mod mpys32h;
#[doc = "MAC32L (rw) register accessor: an alias for `Reg<MAC32L_SPEC>`"]
pub type MAC32L = crate::Reg<mac32l::MAC32L_SPEC>;
#[doc = "32-bit operand 1 multiply accumulate low word"]
pub mod mac32l;
#[doc = "MAC32H (rw) register accessor: an alias for `Reg<MAC32H_SPEC>`"]
pub type MAC32H = crate::Reg<mac32h::MAC32H_SPEC>;
#[doc = "32-bit operand 1 multiply accumulate high word"]
pub mod mac32h;
#[doc = "MACS32L (rw) register accessor: an alias for `Reg<MACS32L_SPEC>`"]
pub type MACS32L = crate::Reg<macs32l::MACS32L_SPEC>;
#[doc = "32-bit operand 1 signed multiply accumulate low word"]
pub mod macs32l;
#[doc = "MACS32H (rw) register accessor: an alias for `Reg<MACS32H_SPEC>`"]
pub type MACS32H = crate::Reg<macs32h::MACS32H_SPEC>;
#[doc = "32-bit operand 1 signed multiply accumulate high word"]
pub mod macs32h;
#[doc = "OP2L (rw) register accessor: an alias for `Reg<OP2L_SPEC>`"]
pub type OP2L = crate::Reg<op2l::OP2L_SPEC>;
#[doc = "32-bit operand 2 low word"]
pub mod op2l;
#[doc = "OP2H (rw) register accessor: an alias for `Reg<OP2H_SPEC>`"]
pub type OP2H = crate::Reg<op2h::OP2H_SPEC>;
#[doc = "32-bit operand 2 high word"]
pub mod op2h;
#[doc = "RES0 (rw) register accessor: an alias for `Reg<RES0_SPEC>`"]
pub type RES0 = crate::Reg<res0::RES0_SPEC>;
#[doc = "32x32-bit result 0 least significant word"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: an alias for `Reg<RES1_SPEC>`"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: an alias for `Reg<RES2_SPEC>`"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: an alias for `Reg<RES3_SPEC>`"]
pub type RES3 = crate::Reg<res3::RES3_SPEC>;
#[doc = "32x32-bit result 3 most significant word"]
pub mod res3;
#[doc = "MPY32CTL0 (rw) register accessor: an alias for `Reg<MPY32CTL0_SPEC>`"]
pub type MPY32CTL0 = crate::Reg<mpy32ctl0::MPY32CTL0_SPEC>;
#[doc = "MPY32 control register 0"]
pub mod mpy32ctl0;
