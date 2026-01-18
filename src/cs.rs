#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csctl0: Csctl0,
    csctl1: Csctl1,
    csctl2: Csctl2,
    csctl3: Csctl3,
    csctl4: Csctl4,
    csctl5: Csctl5,
    csctl6: Csctl6,
    csctl7: Csctl7,
    csctl8: Csctl8,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock System Control 0"]
    #[inline(always)]
    pub const fn csctl0(&self) -> &Csctl0 {
        &self.csctl0
    }
    #[doc = "0x02 - Clock System Control 1"]
    #[inline(always)]
    pub const fn csctl1(&self) -> &Csctl1 {
        &self.csctl1
    }
    #[doc = "0x04 - Clock System Control 2"]
    #[inline(always)]
    pub const fn csctl2(&self) -> &Csctl2 {
        &self.csctl2
    }
    #[doc = "0x06 - Clock System Control 3"]
    #[inline(always)]
    pub const fn csctl3(&self) -> &Csctl3 {
        &self.csctl3
    }
    #[doc = "0x08 - Clock System Control 4"]
    #[inline(always)]
    pub const fn csctl4(&self) -> &Csctl4 {
        &self.csctl4
    }
    #[doc = "0x0a - Clock System Control 5"]
    #[inline(always)]
    pub const fn csctl5(&self) -> &Csctl5 {
        &self.csctl5
    }
    #[doc = "0x0c - Clock System Control 6"]
    #[inline(always)]
    pub const fn csctl6(&self) -> &Csctl6 {
        &self.csctl6
    }
    #[doc = "0x0e - Clock System Control Register 7"]
    #[inline(always)]
    pub const fn csctl7(&self) -> &Csctl7 {
        &self.csctl7
    }
    #[doc = "0x10 - Clock System Control Register 8"]
    #[inline(always)]
    pub const fn csctl8(&self) -> &Csctl8 {
        &self.csctl8
    }
}
#[doc = "CSCTL0 (rw) register accessor: Clock System Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl0`] module"]
#[doc(alias = "CSCTL0")]
pub type Csctl0 = crate::Reg<csctl0::Csctl0Spec>;
#[doc = "Clock System Control 0"]
pub mod csctl0;
#[doc = "CSCTL1 (rw) register accessor: Clock System Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl1`] module"]
#[doc(alias = "CSCTL1")]
pub type Csctl1 = crate::Reg<csctl1::Csctl1Spec>;
#[doc = "Clock System Control 1"]
pub mod csctl1;
#[doc = "CSCTL2 (rw) register accessor: Clock System Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl2`] module"]
#[doc(alias = "CSCTL2")]
pub type Csctl2 = crate::Reg<csctl2::Csctl2Spec>;
#[doc = "Clock System Control 2"]
pub mod csctl2;
#[doc = "CSCTL3 (rw) register accessor: Clock System Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl3`] module"]
#[doc(alias = "CSCTL3")]
pub type Csctl3 = crate::Reg<csctl3::Csctl3Spec>;
#[doc = "Clock System Control 3"]
pub mod csctl3;
#[doc = "CSCTL4 (rw) register accessor: Clock System Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl4`] module"]
#[doc(alias = "CSCTL4")]
pub type Csctl4 = crate::Reg<csctl4::Csctl4Spec>;
#[doc = "Clock System Control 4"]
pub mod csctl4;
#[doc = "CSCTL5 (rw) register accessor: Clock System Control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl5`] module"]
#[doc(alias = "CSCTL5")]
pub type Csctl5 = crate::Reg<csctl5::Csctl5Spec>;
#[doc = "Clock System Control 5"]
pub mod csctl5;
#[doc = "CSCTL6 (rw) register accessor: Clock System Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl6`] module"]
#[doc(alias = "CSCTL6")]
pub type Csctl6 = crate::Reg<csctl6::Csctl6Spec>;
#[doc = "Clock System Control 6"]
pub mod csctl6;
#[doc = "CSCTL7 (rw) register accessor: Clock System Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl7`] module"]
#[doc(alias = "CSCTL7")]
pub type Csctl7 = crate::Reg<csctl7::Csctl7Spec>;
#[doc = "Clock System Control Register 7"]
pub mod csctl7;
#[doc = "CSCTL8 (rw) register accessor: Clock System Control Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl8`] module"]
#[doc(alias = "CSCTL8")]
pub type Csctl8 = crate::Reg<csctl8::Csctl8Spec>;
#[doc = "Clock System Control Register 8"]
pub mod csctl8;
