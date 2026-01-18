#[doc = "Register `P1REN` reader"]
pub type R = crate::R<P1renSpec>;
#[doc = "Register `P1REN` writer"]
pub type W = crate::W<P1renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1renSpec;
impl crate::RegisterSpec for P1renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ren::R`](R) reader structure"]
impl crate::Readable for P1renSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ren::W`](W) writer structure"]
impl crate::Writable for P1renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1REN to value 0"]
impl crate::Resettable for P1renSpec {}
