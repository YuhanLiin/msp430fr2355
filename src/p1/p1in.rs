#[doc = "Register `P1IN` reader"]
pub type R = crate::R<P1inSpec>;
#[doc = "Register `P1IN` writer"]
pub type W = crate::W<P1inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p1in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1inSpec;
impl crate::RegisterSpec for P1inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1in::R`](R) reader structure"]
impl crate::Readable for P1inSpec {}
#[doc = "`write(|w| ..)` method takes [`p1in::W`](W) writer structure"]
impl crate::Writable for P1inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1inSpec {}
