#[doc = "Register `TB0R` reader"]
pub type R = crate::R<Tb0rSpec>;
#[doc = "Register `TB0R` writer"]
pub type W = crate::W<Tb0rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0rSpec;
impl crate::RegisterSpec for Tb0rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0r::R`](R) reader structure"]
impl crate::Readable for Tb0rSpec {}
#[doc = "`write(|w| ..)` method takes [`tb0r::W`](W) writer structure"]
impl crate::Writable for Tb0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB0R to value 0"]
impl crate::Resettable for Tb0rSpec {}
