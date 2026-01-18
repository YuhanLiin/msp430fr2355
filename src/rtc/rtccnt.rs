#[doc = "Register `RTCCNT` reader"]
pub type R = crate::R<RtccntSpec>;
#[doc = "Register `RTCCNT` writer"]
pub type W = crate::W<RtccntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccntSpec;
impl crate::RegisterSpec for RtccntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtccnt::R`](R) reader structure"]
impl crate::Readable for RtccntSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt::W`](W) writer structure"]
impl crate::Writable for RtccntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCNT to value 0"]
impl crate::Resettable for RtccntSpec {}
