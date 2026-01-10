#[doc = "Register `OP2H` reader"]
pub type R = crate::R<Op2hSpec>;
#[doc = "Register `OP2H` writer"]
pub type W = crate::W<Op2hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 2 high word\n\nYou can [`read`](crate::Reg::read) this register and get [`op2h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op2hSpec;
impl crate::RegisterSpec for Op2hSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`op2h::R`](R) reader structure"]
impl crate::Readable for Op2hSpec {}
#[doc = "`write(|w| ..)` method takes [`op2h::W`](W) writer structure"]
impl crate::Writable for Op2hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OP2H to value 0"]
impl crate::Resettable for Op2hSpec {}
