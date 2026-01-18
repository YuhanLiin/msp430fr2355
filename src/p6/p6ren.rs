#[doc = "Register `P6REN` reader"]
pub type R = crate::R<P6renSpec>;
#[doc = "Register `P6REN` writer"]
pub type W = crate::W<P6renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6renSpec;
impl crate::RegisterSpec for P6renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ren::R`](R) reader structure"]
impl crate::Readable for P6renSpec {}
#[doc = "`write(|w| ..)` method takes [`p6ren::W`](W) writer structure"]
impl crate::Writable for P6renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6REN to value 0"]
impl crate::Resettable for P6renSpec {}
