#[doc = "Register `P1IES` reader"]
pub type R = crate::R<P1iesSpec>;
#[doc = "Register `P1IES` writer"]
pub type W = crate::W<P1iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1iesSpec;
impl crate::RegisterSpec for P1iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ies::R`](R) reader structure"]
impl crate::Readable for P1iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ies::W`](W) writer structure"]
impl crate::Writable for P1iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1iesSpec {}
