#[doc = "Register `DCO_TAP_24MHZ` reader"]
pub type R = crate::R<DcoTap24mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO tap setting for 24 MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`dco_tap_24mhz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoTap24mhzSpec;
impl crate::RegisterSpec for DcoTap24mhzSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dco_tap_24mhz::R`](R) reader structure"]
impl crate::Readable for DcoTap24mhzSpec {}
#[doc = "`reset()` method sets DCO_TAP_24MHZ to value 0"]
impl crate::Resettable for DcoTap24mhzSpec {}
