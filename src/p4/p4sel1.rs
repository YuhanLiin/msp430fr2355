#[doc = "Register `P4SEL1` reader"]
pub type R = crate::R<P4sel1Spec>;
#[doc = "Register `P4SEL1` writer"]
pub type W = crate::W<P4sel1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4sel1Spec;
impl crate::RegisterSpec for P4sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4sel1::R`](R) reader structure"]
impl crate::Readable for P4sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p4sel1::W`](W) writer structure"]
impl crate::Writable for P4sel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4SEL1 to value 0"]
impl crate::Resettable for P4sel1Spec {}
