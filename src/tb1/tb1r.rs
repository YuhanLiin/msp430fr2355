#[doc = "Register `TB1R` reader"]
pub type R = crate::R<Tb1rSpec>;
#[doc = "Register `TB1R` writer"]
pub type W = crate::W<Tb1rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb1rSpec;
impl crate::RegisterSpec for Tb1rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb1r::R`](R) reader structure"]
impl crate::Readable for Tb1rSpec {}
#[doc = "`write(|w| ..)` method takes [`tb1r::W`](W) writer structure"]
impl crate::Writable for Tb1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB1R to value 0"]
impl crate::Resettable for Tb1rSpec {}
