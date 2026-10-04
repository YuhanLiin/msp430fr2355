#[doc = "Register `TEMP_30C` reader"]
pub type R = crate::R<Temp30cSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Temperature sensor ADC result at 30 degrees C\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_30c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Temp30cSpec;
impl crate::RegisterSpec for Temp30cSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`temp_30c::R`](R) reader structure"]
impl crate::Readable for Temp30cSpec {}
#[doc = "`reset()` method sets TEMP_30C to value 0"]
impl crate::Resettable for Temp30cSpec {}
