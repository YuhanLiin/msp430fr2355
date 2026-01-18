#[doc = "Register `P5REN` reader"]
pub type R = crate::R<P5renSpec>;
#[doc = "Register `P5REN` writer"]
pub type W = crate::W<P5renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5renSpec;
impl crate::RegisterSpec for P5renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ren::R`](R) reader structure"]
impl crate::Readable for P5renSpec {}
#[doc = "`write(|w| ..)` method takes [`p5ren::W`](W) writer structure"]
impl crate::Writable for P5renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5REN to value 0"]
impl crate::Resettable for P5renSpec {}
