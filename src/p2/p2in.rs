#[doc = "Register `P2IN` reader"]
pub type R = crate::R<P2inSpec>;
#[doc = "Register `P2IN` writer"]
pub type W = crate::W<P2inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p2in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2inSpec;
impl crate::RegisterSpec for P2inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2in::R`](R) reader structure"]
impl crate::Readable for P2inSpec {}
#[doc = "`write(|w| ..)` method takes [`p2in::W`](W) writer structure"]
impl crate::Writable for P2inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2IN to value 0"]
impl crate::Resettable for P2inSpec {}
