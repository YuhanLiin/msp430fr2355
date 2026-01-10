#[doc = "Register `MAC` reader"]
pub type R = crate::R<MacSpec>;
#[doc = "Register `MAC` writer"]
pub type W = crate::W<MacSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16-bit operand one multiply accumulate\n\nYou can [`read`](crate::Reg::read) this register and get [`mac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacSpec;
impl crate::RegisterSpec for MacSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mac::R`](R) reader structure"]
impl crate::Readable for MacSpec {}
#[doc = "`write(|w| ..)` method takes [`mac::W`](W) writer structure"]
impl crate::Writable for MacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC to value 0"]
impl crate::Resettable for MacSpec {}
