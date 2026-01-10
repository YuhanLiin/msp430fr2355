#[doc = "Register `SAC0DACSTS` reader"]
pub type R = crate::R<Sac0dacstsSpec>;
#[doc = "Register `SAC0DACSTS` writer"]
pub type W = crate::W<Sac0dacstsSpec>;
#[doc = "Field `DACIFG` reader - SAC DAC data update flag"]
pub type DacifgR = crate::BitReader;
#[doc = "Field `DACIFG` writer - SAC DAC data update flag"]
pub type DacifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&self) -> DacifgR {
        DacifgR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&mut self) -> DacifgW<'_, Sac0dacstsSpec> {
        DacifgW::new(self, 0)
    }
}
#[doc = "SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0dacsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0dacsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac0dacstsSpec;
impl crate::RegisterSpec for Sac0dacstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac0dacsts::R`](R) reader structure"]
impl crate::Readable for Sac0dacstsSpec {}
#[doc = "`write(|w| ..)` method takes [`sac0dacsts::W`](W) writer structure"]
impl crate::Writable for Sac0dacstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC0DACSTS to value 0"]
impl crate::Resettable for Sac0dacstsSpec {}
