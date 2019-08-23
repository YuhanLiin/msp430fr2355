#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power mode 5 control register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "Power mode 5 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pm5ctl0](pm5ctl0) module"]
pub type PM5CTL0 = crate::Reg<u16, _PM5CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PM5CTL0;
#[doc = "`read()` method returns [pm5ctl0::R](pm5ctl0::R) reader structure"]
impl crate::Readable for PM5CTL0 {}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](pm5ctl0::W) writer structure"]
impl crate::Writable for PM5CTL0 {}
#[doc = "Power mode 5 control register 0"]
pub mod pm5ctl0;
