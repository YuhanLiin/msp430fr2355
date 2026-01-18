#[doc = "Register `TB3R` reader"]
pub type R = crate::R<Tb3rSpec>;
#[doc = "Register `TB3R` writer"]
pub type W = crate::W<Tb3rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb3rSpec;
impl crate::RegisterSpec for Tb3rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb3r::R`](R) reader structure"]
impl crate::Readable for Tb3rSpec {}
#[doc = "`write(|w| ..)` method takes [`tb3r::W`](W) writer structure"]
impl crate::Writable for Tb3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB3R to value 0"]
impl crate::Resettable for Tb3rSpec {}
