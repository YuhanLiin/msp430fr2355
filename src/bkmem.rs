#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bakmem0: Bakmem0,
    bakmem1: Bakmem1,
    bakmem2: Bakmem2,
    bakmem3: Bakmem3,
    bakmem4: Bakmem4,
    bakmem5: Bakmem5,
    bakmem6: Bakmem6,
    bakmem7: Bakmem7,
    bakmem8: Bakmem8,
    bakmem9: Bakmem9,
    bakmem10: Bakmem10,
    bakmem11: Bakmem11,
    bakmem12: Bakmem12,
    bakmem13: Bakmem13,
    bakmem14: Bakmem14,
    bakmem15: Bakmem15,
}
impl RegisterBlock {
    #[doc = "0x00 - Backup Memory registers. Backup Memory 0."]
    #[inline(always)]
    pub const fn bakmem0(&self) -> &Bakmem0 {
        &self.bakmem0
    }
    #[doc = "0x02 - Backup Memory 1."]
    #[inline(always)]
    pub const fn bakmem1(&self) -> &Bakmem1 {
        &self.bakmem1
    }
    #[doc = "0x04 - Backup Memory 2."]
    #[inline(always)]
    pub const fn bakmem2(&self) -> &Bakmem2 {
        &self.bakmem2
    }
    #[doc = "0x06 - Backup Memory 3."]
    #[inline(always)]
    pub const fn bakmem3(&self) -> &Bakmem3 {
        &self.bakmem3
    }
    #[doc = "0x08 - Backup Memory 4."]
    #[inline(always)]
    pub const fn bakmem4(&self) -> &Bakmem4 {
        &self.bakmem4
    }
    #[doc = "0x0a - Backup Memory 5."]
    #[inline(always)]
    pub const fn bakmem5(&self) -> &Bakmem5 {
        &self.bakmem5
    }
    #[doc = "0x0c - Backup Memory 6."]
    #[inline(always)]
    pub const fn bakmem6(&self) -> &Bakmem6 {
        &self.bakmem6
    }
    #[doc = "0x0e - Backup Memory 7."]
    #[inline(always)]
    pub const fn bakmem7(&self) -> &Bakmem7 {
        &self.bakmem7
    }
    #[doc = "0x10 - Backup Memory 8."]
    #[inline(always)]
    pub const fn bakmem8(&self) -> &Bakmem8 {
        &self.bakmem8
    }
    #[doc = "0x12 - Backup Memory 9."]
    #[inline(always)]
    pub const fn bakmem9(&self) -> &Bakmem9 {
        &self.bakmem9
    }
    #[doc = "0x14 - Backup Memory registers. Backup Memory 10."]
    #[inline(always)]
    pub const fn bakmem10(&self) -> &Bakmem10 {
        &self.bakmem10
    }
    #[doc = "0x16 - Backup Memory 11."]
    #[inline(always)]
    pub const fn bakmem11(&self) -> &Bakmem11 {
        &self.bakmem11
    }
    #[doc = "0x18 - Backup Memory 12."]
    #[inline(always)]
    pub const fn bakmem12(&self) -> &Bakmem12 {
        &self.bakmem12
    }
    #[doc = "0x1a - Backup Memory 13."]
    #[inline(always)]
    pub const fn bakmem13(&self) -> &Bakmem13 {
        &self.bakmem13
    }
    #[doc = "0x1c - Backup Memory 14."]
    #[inline(always)]
    pub const fn bakmem14(&self) -> &Bakmem14 {
        &self.bakmem14
    }
    #[doc = "0x1e - Backup Memory 15."]
    #[inline(always)]
    pub const fn bakmem15(&self) -> &Bakmem15 {
        &self.bakmem15
    }
}
#[doc = "BAKMEM0 (rw) register accessor: Backup Memory registers. Backup Memory 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem0`] module"]
#[doc(alias = "BAKMEM0")]
pub type Bakmem0 = crate::Reg<bakmem0::Bakmem0Spec>;
#[doc = "Backup Memory registers. Backup Memory 0."]
pub mod bakmem0;
#[doc = "BAKMEM1 (rw) register accessor: Backup Memory 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem1`] module"]
#[doc(alias = "BAKMEM1")]
pub type Bakmem1 = crate::Reg<bakmem1::Bakmem1Spec>;
#[doc = "Backup Memory 1."]
pub mod bakmem1;
#[doc = "BAKMEM2 (rw) register accessor: Backup Memory 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem2`] module"]
#[doc(alias = "BAKMEM2")]
pub type Bakmem2 = crate::Reg<bakmem2::Bakmem2Spec>;
#[doc = "Backup Memory 2."]
pub mod bakmem2;
#[doc = "BAKMEM3 (rw) register accessor: Backup Memory 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem3`] module"]
#[doc(alias = "BAKMEM3")]
pub type Bakmem3 = crate::Reg<bakmem3::Bakmem3Spec>;
#[doc = "Backup Memory 3."]
pub mod bakmem3;
#[doc = "BAKMEM4 (rw) register accessor: Backup Memory 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem4`] module"]
#[doc(alias = "BAKMEM4")]
pub type Bakmem4 = crate::Reg<bakmem4::Bakmem4Spec>;
#[doc = "Backup Memory 4."]
pub mod bakmem4;
#[doc = "BAKMEM5 (rw) register accessor: Backup Memory 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem5`] module"]
#[doc(alias = "BAKMEM5")]
pub type Bakmem5 = crate::Reg<bakmem5::Bakmem5Spec>;
#[doc = "Backup Memory 5."]
pub mod bakmem5;
#[doc = "BAKMEM6 (rw) register accessor: Backup Memory 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem6`] module"]
#[doc(alias = "BAKMEM6")]
pub type Bakmem6 = crate::Reg<bakmem6::Bakmem6Spec>;
#[doc = "Backup Memory 6."]
pub mod bakmem6;
#[doc = "BAKMEM7 (rw) register accessor: Backup Memory 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem7`] module"]
#[doc(alias = "BAKMEM7")]
pub type Bakmem7 = crate::Reg<bakmem7::Bakmem7Spec>;
#[doc = "Backup Memory 7."]
pub mod bakmem7;
#[doc = "BAKMEM8 (rw) register accessor: Backup Memory 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem8`] module"]
#[doc(alias = "BAKMEM8")]
pub type Bakmem8 = crate::Reg<bakmem8::Bakmem8Spec>;
#[doc = "Backup Memory 8."]
pub mod bakmem8;
#[doc = "BAKMEM9 (rw) register accessor: Backup Memory 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem9`] module"]
#[doc(alias = "BAKMEM9")]
pub type Bakmem9 = crate::Reg<bakmem9::Bakmem9Spec>;
#[doc = "Backup Memory 9."]
pub mod bakmem9;
#[doc = "BAKMEM10 (rw) register accessor: Backup Memory registers. Backup Memory 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem10`] module"]
#[doc(alias = "BAKMEM10")]
pub type Bakmem10 = crate::Reg<bakmem10::Bakmem10Spec>;
#[doc = "Backup Memory registers. Backup Memory 10."]
pub mod bakmem10;
#[doc = "BAKMEM11 (rw) register accessor: Backup Memory 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem11`] module"]
#[doc(alias = "BAKMEM11")]
pub type Bakmem11 = crate::Reg<bakmem11::Bakmem11Spec>;
#[doc = "Backup Memory 11."]
pub mod bakmem11;
#[doc = "BAKMEM12 (rw) register accessor: Backup Memory 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem12`] module"]
#[doc(alias = "BAKMEM12")]
pub type Bakmem12 = crate::Reg<bakmem12::Bakmem12Spec>;
#[doc = "Backup Memory 12."]
pub mod bakmem12;
#[doc = "BAKMEM13 (rw) register accessor: Backup Memory 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem13`] module"]
#[doc(alias = "BAKMEM13")]
pub type Bakmem13 = crate::Reg<bakmem13::Bakmem13Spec>;
#[doc = "Backup Memory 13."]
pub mod bakmem13;
#[doc = "BAKMEM14 (rw) register accessor: Backup Memory 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem14`] module"]
#[doc(alias = "BAKMEM14")]
pub type Bakmem14 = crate::Reg<bakmem14::Bakmem14Spec>;
#[doc = "Backup Memory 14."]
pub mod bakmem14;
#[doc = "BAKMEM15 (rw) register accessor: Backup Memory 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem15`] module"]
#[doc(alias = "BAKMEM15")]
pub type Bakmem15 = crate::Reg<bakmem15::Bakmem15Spec>;
#[doc = "Backup Memory 15."]
pub mod bakmem15;
