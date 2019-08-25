#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    pub wdtctl: WDTCTL,
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtctl](wdtctl) module"]
pub type WDTCTL = crate::Reg<u16, _WDTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCTL;
#[doc = "`read()` method returns [wdtctl::R](wdtctl::R) reader structure"]
impl crate::Readable for WDTCTL {}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](wdtctl::W) writer structure"]
impl crate::Writable for WDTCTL {}
#[doc = "Watchdog Timer Control Register"]
pub mod wdtctl;
