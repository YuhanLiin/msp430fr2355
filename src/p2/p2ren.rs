#[doc = "Register `P2REN` reader"]
pub type R = crate::R<P2renSpec>;
#[doc = "Register `P2REN` writer"]
pub type W = crate::W<P2renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2renSpec;
impl crate::RegisterSpec for P2renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ren::R`](R) reader structure"]
impl crate::Readable for P2renSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ren::W`](W) writer structure"]
impl crate::Writable for P2renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2REN to value 0"]
impl crate::Resettable for P2renSpec {}
