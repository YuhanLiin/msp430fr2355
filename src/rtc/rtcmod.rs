#[doc = "Register `RTCMOD` reader"]
pub type R = crate::R<RtcmodSpec>;
#[doc = "Register `RTCMOD` writer"]
pub type W = crate::W<RtcmodSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC Counter Modulo Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcmodSpec;
impl crate::RegisterSpec for RtcmodSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcmod::R`](R) reader structure"]
impl crate::Readable for RtcmodSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcmod::W`](W) writer structure"]
impl crate::Writable for RtcmodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCMOD to value 0"]
impl crate::Resettable for RtcmodSpec {}
