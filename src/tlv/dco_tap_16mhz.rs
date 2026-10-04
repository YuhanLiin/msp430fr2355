#[doc = "Register `DCO_TAP_16MHZ` reader"]
pub type R = crate::R<DcoTap16mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO tap setting for 16 MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`dco_tap_16mhz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoTap16mhzSpec;
impl crate::RegisterSpec for DcoTap16mhzSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dco_tap_16mhz::R`](R) reader structure"]
impl crate::Readable for DcoTap16mhzSpec {}
#[doc = "`reset()` method sets DCO_TAP_16MHZ to value 0"]
impl crate::Resettable for DcoTap16mhzSpec {}
