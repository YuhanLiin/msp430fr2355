#[doc = "Register `MACS` reader"]
pub type R = crate::R<MacsSpec>;
#[doc = "Register `MACS` writer"]
pub type W = crate::W<MacsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16-bit operand one signed multiply accumulate\n\nYou can [`read`](crate::Reg::read) this register and get [`macs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacsSpec;
impl crate::RegisterSpec for MacsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`macs::R`](R) reader structure"]
impl crate::Readable for MacsSpec {}
#[doc = "`write(|w| ..)` method takes [`macs::W`](W) writer structure"]
impl crate::Writable for MacsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACS to value 0"]
impl crate::Resettable for MacsSpec {}
