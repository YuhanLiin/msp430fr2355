#[doc = "Register `P2IES` reader"]
pub type R = crate::R<P2iesSpec>;
#[doc = "Register `P2IES` writer"]
pub type W = crate::W<P2iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2iesSpec;
impl crate::RegisterSpec for P2iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ies::R`](R) reader structure"]
impl crate::Readable for P2iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ies::W`](W) writer structure"]
impl crate::Writable for P2iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2IES to value 0"]
impl crate::Resettable for P2iesSpec {}
