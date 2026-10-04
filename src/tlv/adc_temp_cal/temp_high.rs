#[doc = "Register `TEMP_HIGH` reader"]
pub type R = crate::R<TempHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Temperature sensor ADC result at 105 degrees C\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_high::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempHighSpec;
impl crate::RegisterSpec for TempHighSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`temp_high::R`](R) reader structure"]
impl crate::Readable for TempHighSpec {}
#[doc = "`reset()` method sets TEMP_HIGH to value 0"]
impl crate::Resettable for TempHighSpec {}
