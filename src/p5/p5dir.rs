#[doc = "Register `P5DIR` reader"]
pub type R = crate::R<P5dirSpec>;
#[doc = "Register `P5DIR` writer"]
pub type W = crate::W<P5dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p5dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5dirSpec;
impl crate::RegisterSpec for P5dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5dir::R`](R) reader structure"]
impl crate::Readable for P5dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p5dir::W`](W) writer structure"]
impl crate::Writable for P5dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5DIR to value 0"]
impl crate::Resettable for P5dirSpec {}
