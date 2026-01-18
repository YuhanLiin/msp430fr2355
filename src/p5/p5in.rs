#[doc = "Register `P5IN` reader"]
pub type R = crate::R<P5inSpec>;
#[doc = "Register `P5IN` writer"]
pub type W = crate::W<P5inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p5in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5inSpec;
impl crate::RegisterSpec for P5inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5in::R`](R) reader structure"]
impl crate::Readable for P5inSpec {}
#[doc = "`write(|w| ..)` method takes [`p5in::W`](W) writer structure"]
impl crate::Writable for P5inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5IN to value 0"]
impl crate::Resettable for P5inSpec {}
