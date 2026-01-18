#[doc = "Register `TB2R` reader"]
pub type R = crate::R<Tb2rSpec>;
#[doc = "Register `TB2R` writer"]
pub type W = crate::W<Tb2rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb2rSpec;
impl crate::RegisterSpec for Tb2rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb2r::R`](R) reader structure"]
impl crate::Readable for Tb2rSpec {}
#[doc = "`write(|w| ..)` method takes [`tb2r::W`](W) writer structure"]
impl crate::Writable for Tb2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB2R to value 0"]
impl crate::Resettable for Tb2rSpec {}
