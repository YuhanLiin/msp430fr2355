#[doc = "Register `SAC2DAT` reader"]
pub type R = crate::R<Sac2datSpec>;
#[doc = "Register `SAC2DAT` writer"]
pub type W = crate::W<Sac2datSpec>;
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
    pub fn dacdata(&mut self) -> DacdataW<'_, Sac2datSpec> {
        DacdataW::new(self, 0)
    }
}
#[doc = "SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac2datSpec;
impl crate::RegisterSpec for Sac2datSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac2dat::R`](R) reader structure"]
impl crate::Readable for Sac2datSpec {}
#[doc = "`write(|w| ..)` method takes [`sac2dat::W`](W) writer structure"]
impl crate::Writable for Sac2datSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC2DAT to value 0"]
impl crate::Resettable for Sac2datSpec {}
