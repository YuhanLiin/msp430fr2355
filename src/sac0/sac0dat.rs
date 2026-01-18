#[doc = "Register `SAC0DAT` reader"]
pub type R = crate::R<Sac0datSpec>;
#[doc = "Register `SAC0DAT` writer"]
pub type W = crate::W<Sac0datSpec>;
#[doc = "Field `DACData` reader - SAC DAC data in unsigned format."]
pub type DacdataR = crate::FieldReader<u16>;
#[doc = "Field `DACData` writer - SAC DAC data in unsigned format."]
pub type DacdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - SAC DAC data in unsigned format."]
    #[inline(always)]
    pub fn dacdata(&self) -> DacdataR {
        DacdataR::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - SAC DAC data in unsigned format."]
    #[inline(always)]
    pub fn dacdata(&mut self) -> DacdataW<'_, Sac0datSpec> {
        DacdataW::new(self, 0)
    }
}
#[doc = "SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac0datSpec;
impl crate::RegisterSpec for Sac0datSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac0dat::R`](R) reader structure"]
impl crate::Readable for Sac0datSpec {}
#[doc = "`write(|w| ..)` method takes [`sac0dat::W`](W) writer structure"]
impl crate::Writable for Sac0datSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC0DAT to value 0"]
impl crate::Resettable for Sac0datSpec {}
