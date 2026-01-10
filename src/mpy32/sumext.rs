#[doc = "Register `SUMEXT` reader"]
pub type R = crate::R<SumextSpec>;
#[doc = "Register `SUMEXT` writer"]
pub type W = crate::W<SumextSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16x16-bit sum extension register\n\nYou can [`read`](crate::Reg::read) this register and get [`sumext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sumext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SumextSpec;
impl crate::RegisterSpec for SumextSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sumext::R`](R) reader structure"]
impl crate::Readable for SumextSpec {}
#[doc = "`write(|w| ..)` method takes [`sumext::W`](W) writer structure"]
impl crate::Writable for SumextSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUMEXT to value 0"]
impl crate::Resettable for SumextSpec {}
