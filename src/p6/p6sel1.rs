#[doc = "Register `P6SEL1` reader"]
pub type R = crate::R<P6sel1Spec>;
#[doc = "Register `P6SEL1` writer"]
pub type W = crate::W<P6sel1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p6sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6sel1Spec;
impl crate::RegisterSpec for P6sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6sel1::R`](R) reader structure"]
impl crate::Readable for P6sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p6sel1::W`](W) writer structure"]
impl crate::Writable for P6sel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6SEL1 to value 0"]
impl crate::Resettable for P6sel1Spec {}
