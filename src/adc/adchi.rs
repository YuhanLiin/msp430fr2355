#[doc = "Register `ADCHI` reader"]
pub type R = crate::R<AdchiSpec>;
#[doc = "Register `ADCHI` writer"]
pub type W = crate::W<AdchiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adchi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adchi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdchiSpec;
impl crate::RegisterSpec for AdchiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adchi::R`](R) reader structure"]
impl crate::Readable for AdchiSpec {}
#[doc = "`write(|w| ..)` method takes [`adchi::W`](W) writer structure"]
impl crate::Writable for AdchiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCHI to value 0"]
impl crate::Resettable for AdchiSpec {}
