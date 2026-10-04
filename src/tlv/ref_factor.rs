#[doc = "Register `REF_FACTOR[%s]` reader"]
pub type R = crate::R<RefFactorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Factor of the 1.5 V, 2.0 V or 2.5 V reference (index 0, 1 or 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_factor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefFactorSpec;
impl crate::RegisterSpec for RefFactorSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ref_factor::R`](R) reader structure"]
impl crate::Readable for RefFactorSpec {}
#[doc = "`reset()` method sets REF_FACTOR[%s] to value 0"]
impl crate::Resettable for RefFactorSpec {}
