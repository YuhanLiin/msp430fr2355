#[doc = "Register `P4REN` reader"]
pub type R = crate::R<P4renSpec>;
#[doc = "Register `P4REN` writer"]
pub type W = crate::W<P4renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4renSpec;
impl crate::RegisterSpec for P4renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ren::R`](R) reader structure"]
impl crate::Readable for P4renSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ren::W`](W) writer structure"]
impl crate::Writable for P4renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4REN to value 0"]
impl crate::Resettable for P4renSpec {}
