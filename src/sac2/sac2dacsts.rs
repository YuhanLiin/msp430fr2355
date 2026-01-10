#[doc = "Register `SAC2DACSTS` reader"]
pub type R = crate::R<Sac2dacstsSpec>;
#[doc = "Register `SAC2DACSTS` writer"]
pub type W = crate::W<Sac2dacstsSpec>;
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
    pub fn dacifg(&mut self) -> DacifgW<'_, Sac2dacstsSpec> {
        DacifgW::new(self, 0)
    }
}
#[doc = "SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2dacsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2dacsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac2dacstsSpec;
impl crate::RegisterSpec for Sac2dacstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac2dacsts::R`](R) reader structure"]
impl crate::Readable for Sac2dacstsSpec {}
#[doc = "`write(|w| ..)` method takes [`sac2dacsts::W`](W) writer structure"]
impl crate::Writable for Sac2dacstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC2DACSTS to value 0"]
impl crate::Resettable for Sac2dacstsSpec {}
