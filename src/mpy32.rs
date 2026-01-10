#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpy: Mpy,
    mpys: Mpys,
    mac: Mac,
    macs: Macs,
    op2: Op2,
    reslo: Reslo,
    reshi: Reshi,
    sumext: Sumext,
    mpy32l: Mpy32l,
    mpy32h: Mpy32h,
    mpys32l: Mpys32l,
    mpys32h: Mpys32h,
    mac32l: Mac32l,
    mac32h: Mac32h,
    macs32l: Macs32l,
    macs32h: Macs32h,
    op2l: Op2l,
    op2h: Op2h,
    res0: Res0,
    res1: Res1,
    res2: Res2,
    res3: Res3,
    mpy32ctl0: Mpy32ctl0,
}
impl RegisterBlock {
    #[doc = "0x00 - 16-bit operand one multiply"]
    #[inline(always)]
    pub const fn mpy(&self) -> &Mpy {
        &self.mpy
    }
    #[doc = "0x02 - 16-bit operand one signed multiply"]
    #[inline(always)]
    pub const fn mpys(&self) -> &Mpys {
        &self.mpys
    }
    #[doc = "0x04 - 16-bit operand one multiply accumulate"]
    #[inline(always)]
    pub const fn mac(&self) -> &Mac {
        &self.mac
    }
    #[doc = "0x06 - 16-bit operand one signed multiply accumulate"]
    #[inline(always)]
    pub const fn macs(&self) -> &Macs {
        &self.macs
    }
    #[doc = "0x08 - 16-bit operand two"]
    #[inline(always)]
    pub const fn op2(&self) -> &Op2 {
        &self.op2
    }
    #[doc = "0x0a - 16x16-bit result low word"]
    #[inline(always)]
    pub const fn reslo(&self) -> &Reslo {
        &self.reslo
    }
    #[doc = "0x0c - 16x16-bit result high word"]
    #[inline(always)]
    pub const fn reshi(&self) -> &Reshi {
        &self.reshi
    }
    #[doc = "0x0e - 16x16-bit sum extension register"]
    #[inline(always)]
    pub const fn sumext(&self) -> &Sumext {
        &self.sumext
    }
    #[doc = "0x10 - 32-bit operand 1 multiply low word"]
    #[inline(always)]
    pub const fn mpy32l(&self) -> &Mpy32l {
        &self.mpy32l
    }
    #[doc = "0x12 - 32-bit operand 1 multiply high word"]
    #[inline(always)]
    pub const fn mpy32h(&self) -> &Mpy32h {
        &self.mpy32h
    }
    #[doc = "0x14 - 32-bit operand 1 signed multiply low word"]
    #[inline(always)]
    pub const fn mpys32l(&self) -> &Mpys32l {
        &self.mpys32l
    }
    #[doc = "0x16 - 32-bit operand 1 signed multiply high word"]
    #[inline(always)]
    pub const fn mpys32h(&self) -> &Mpys32h {
        &self.mpys32h
    }
    #[doc = "0x18 - 32-bit operand 1 multiply accumulate low word"]
    #[inline(always)]
    pub const fn mac32l(&self) -> &Mac32l {
        &self.mac32l
    }
    #[doc = "0x1a - 32-bit operand 1 multiply accumulate high word"]
    #[inline(always)]
    pub const fn mac32h(&self) -> &Mac32h {
        &self.mac32h
    }
    #[doc = "0x1c - 32-bit operand 1 signed multiply accumulate low word"]
    #[inline(always)]
    pub const fn macs32l(&self) -> &Macs32l {
        &self.macs32l
    }
    #[doc = "0x1e - 32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub const fn macs32h(&self) -> &Macs32h {
        &self.macs32h
    }
    #[doc = "0x20 - 32-bit operand 2 low word"]
    #[inline(always)]
    pub const fn op2l(&self) -> &Op2l {
        &self.op2l
    }
    #[doc = "0x22 - 32-bit operand 2 high word"]
    #[inline(always)]
    pub const fn op2h(&self) -> &Op2h {
        &self.op2h
    }
    #[doc = "0x24 - 32x32-bit result 0 least significant word"]
    #[inline(always)]
    pub const fn res0(&self) -> &Res0 {
        &self.res0
    }
    #[doc = "0x26 - 32x32-bit result 1"]
    #[inline(always)]
    pub const fn res1(&self) -> &Res1 {
        &self.res1
    }
    #[doc = "0x28 - 32x32-bit result 2"]
    #[inline(always)]
    pub const fn res2(&self) -> &Res2 {
        &self.res2
    }
    #[doc = "0x2a - 32x32-bit result 3 most significant word"]
    #[inline(always)]
    pub const fn res3(&self) -> &Res3 {
        &self.res3
    }
    #[doc = "0x2c - MPY32 control register 0"]
    #[inline(always)]
    pub const fn mpy32ctl0(&self) -> &Mpy32ctl0 {
        &self.mpy32ctl0
    }
}
#[doc = "MPY (rw) register accessor: 16-bit operand one multiply\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy`] module"]
#[doc(alias = "MPY")]
pub type Mpy = crate::Reg<mpy::MpySpec>;
#[doc = "16-bit operand one multiply"]
pub mod mpy;
#[doc = "MPYS (rw) register accessor: 16-bit operand one signed multiply\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys`] module"]
#[doc(alias = "MPYS")]
pub type Mpys = crate::Reg<mpys::MpysSpec>;
#[doc = "16-bit operand one signed multiply"]
pub mod mpys;
#[doc = "MAC (rw) register accessor: 16-bit operand one multiply accumulate\n\nYou can [`read`](crate::Reg::read) this register and get [`mac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac`] module"]
#[doc(alias = "MAC")]
pub type Mac = crate::Reg<mac::MacSpec>;
#[doc = "16-bit operand one multiply accumulate"]
pub mod mac;
#[doc = "MACS (rw) register accessor: 16-bit operand one signed multiply accumulate\n\nYou can [`read`](crate::Reg::read) this register and get [`macs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs`] module"]
#[doc(alias = "MACS")]
pub type Macs = crate::Reg<macs::MacsSpec>;
#[doc = "16-bit operand one signed multiply accumulate"]
pub mod macs;
#[doc = "OP2 (rw) register accessor: 16-bit operand two\n\nYou can [`read`](crate::Reg::read) this register and get [`op2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2`] module"]
#[doc(alias = "OP2")]
pub type Op2 = crate::Reg<op2::Op2Spec>;
#[doc = "16-bit operand two"]
pub mod op2;
#[doc = "RESLO (rw) register accessor: 16x16-bit result low word\n\nYou can [`read`](crate::Reg::read) this register and get [`reslo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reslo`] module"]
#[doc(alias = "RESLO")]
pub type Reslo = crate::Reg<reslo::ResloSpec>;
#[doc = "16x16-bit result low word"]
pub mod reslo;
#[doc = "RESHI (rw) register accessor: 16x16-bit result high word\n\nYou can [`read`](crate::Reg::read) this register and get [`reshi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reshi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reshi`] module"]
#[doc(alias = "RESHI")]
pub type Reshi = crate::Reg<reshi::ReshiSpec>;
#[doc = "16x16-bit result high word"]
pub mod reshi;
#[doc = "SUMEXT (rw) register accessor: 16x16-bit sum extension register\n\nYou can [`read`](crate::Reg::read) this register and get [`sumext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sumext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sumext`] module"]
#[doc(alias = "SUMEXT")]
pub type Sumext = crate::Reg<sumext::SumextSpec>;
#[doc = "16x16-bit sum extension register"]
pub mod sumext;
#[doc = "MPY32L (rw) register accessor: 32-bit operand 1 multiply low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32l`] module"]
#[doc(alias = "MPY32L")]
pub type Mpy32l = crate::Reg<mpy32l::Mpy32lSpec>;
#[doc = "32-bit operand 1 multiply low word"]
pub mod mpy32l;
#[doc = "MPY32H (rw) register accessor: 32-bit operand 1 multiply high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32h`] module"]
#[doc(alias = "MPY32H")]
pub type Mpy32h = crate::Reg<mpy32h::Mpy32hSpec>;
#[doc = "32-bit operand 1 multiply high word"]
pub mod mpy32h;
#[doc = "MPYS32L (rw) register accessor: 32-bit operand 1 signed multiply low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys32l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys32l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys32l`] module"]
#[doc(alias = "MPYS32L")]
pub type Mpys32l = crate::Reg<mpys32l::Mpys32lSpec>;
#[doc = "32-bit operand 1 signed multiply low word"]
pub mod mpys32l;
#[doc = "MPYS32H (rw) register accessor: 32-bit operand 1 signed multiply high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys32h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys32h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys32h`] module"]
#[doc(alias = "MPYS32H")]
pub type Mpys32h = crate::Reg<mpys32h::Mpys32hSpec>;
#[doc = "32-bit operand 1 signed multiply high word"]
pub mod mpys32h;
#[doc = "MAC32L (rw) register accessor: 32-bit operand 1 multiply accumulate low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mac32l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac32l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac32l`] module"]
#[doc(alias = "MAC32L")]
pub type Mac32l = crate::Reg<mac32l::Mac32lSpec>;
#[doc = "32-bit operand 1 multiply accumulate low word"]
pub mod mac32l;
#[doc = "MAC32H (rw) register accessor: 32-bit operand 1 multiply accumulate high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mac32h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac32h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac32h`] module"]
#[doc(alias = "MAC32H")]
pub type Mac32h = crate::Reg<mac32h::Mac32hSpec>;
#[doc = "32-bit operand 1 multiply accumulate high word"]
pub mod mac32h;
#[doc = "MACS32L (rw) register accessor: 32-bit operand 1 signed multiply accumulate low word\n\nYou can [`read`](crate::Reg::read) this register and get [`macs32l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs32l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs32l`] module"]
#[doc(alias = "MACS32L")]
pub type Macs32l = crate::Reg<macs32l::Macs32lSpec>;
#[doc = "32-bit operand 1 signed multiply accumulate low word"]
pub mod macs32l;
#[doc = "MACS32H (rw) register accessor: 32-bit operand 1 signed multiply accumulate high word\n\nYou can [`read`](crate::Reg::read) this register and get [`macs32h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs32h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs32h`] module"]
#[doc(alias = "MACS32H")]
pub type Macs32h = crate::Reg<macs32h::Macs32hSpec>;
#[doc = "32-bit operand 1 signed multiply accumulate high word"]
pub mod macs32h;
#[doc = "OP2L (rw) register accessor: 32-bit operand 2 low word\n\nYou can [`read`](crate::Reg::read) this register and get [`op2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2l`] module"]
#[doc(alias = "OP2L")]
pub type Op2l = crate::Reg<op2l::Op2lSpec>;
#[doc = "32-bit operand 2 low word"]
pub mod op2l;
#[doc = "OP2H (rw) register accessor: 32-bit operand 2 high word\n\nYou can [`read`](crate::Reg::read) this register and get [`op2h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2h`] module"]
#[doc(alias = "OP2H")]
pub type Op2h = crate::Reg<op2h::Op2hSpec>;
#[doc = "32-bit operand 2 high word"]
pub mod op2h;
#[doc = "RES0 (rw) register accessor: 32x32-bit result 0 least significant word\n\nYou can [`read`](crate::Reg::read) this register and get [`res0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res0`] module"]
#[doc(alias = "RES0")]
pub type Res0 = crate::Reg<res0::Res0Spec>;
#[doc = "32x32-bit result 0 least significant word"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: 32x32-bit result 1\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res1`] module"]
#[doc(alias = "RES1")]
pub type Res1 = crate::Reg<res1::Res1Spec>;
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: 32x32-bit result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res2`] module"]
#[doc(alias = "RES2")]
pub type Res2 = crate::Reg<res2::Res2Spec>;
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: 32x32-bit result 3 most significant word\n\nYou can [`read`](crate::Reg::read) this register and get [`res3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res3`] module"]
#[doc(alias = "RES3")]
pub type Res3 = crate::Reg<res3::Res3Spec>;
#[doc = "32x32-bit result 3 most significant word"]
pub mod res3;
#[doc = "MPY32CTL0 (rw) register accessor: MPY32 control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32ctl0`] module"]
#[doc(alias = "MPY32CTL0")]
pub type Mpy32ctl0 = crate::Reg<mpy32ctl0::Mpy32ctl0Spec>;
#[doc = "MPY32 control register 0"]
pub mod mpy32ctl0;
