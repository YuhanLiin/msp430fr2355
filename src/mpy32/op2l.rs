#[doc = "Register `OP2L` reader"]
pub type R = crate::R<Op2lSpec>;
#[doc = "Register `OP2L` writer"]
pub type W = crate::W<Op2lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 2 low word\n\nYou can [`read`](crate::Reg::read) this register and get [`op2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op2lSpec;
impl crate::RegisterSpec for Op2lSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`op2l::R`](R) reader structure"]
impl crate::Readable for Op2lSpec {}
#[doc = "`write(|w| ..)` method takes [`op2l::W`](W) writer structure"]
impl crate::Writable for Op2lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OP2L to value 0"]
impl crate::Resettable for Op2lSpec {}
