#[doc = "Register `P3REN` reader"]
pub type R = crate::R<P3renSpec>;
#[doc = "Register `P3REN` writer"]
pub type W = crate::W<P3renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3renSpec;
impl crate::RegisterSpec for P3renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ren::R`](R) reader structure"]
impl crate::Readable for P3renSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ren::W`](W) writer structure"]
impl crate::Writable for P3renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3REN to value 0"]
impl crate::Resettable for P3renSpec {}
