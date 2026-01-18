#[doc = "Register `OP2` reader"]
pub type R = crate::R<Op2Spec>;
#[doc = "Register `OP2` writer"]
pub type W = crate::W<Op2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16-bit operand two\n\nYou can [`read`](crate::Reg::read) this register and get [`op2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op2Spec;
impl crate::RegisterSpec for Op2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`op2::R`](R) reader structure"]
impl crate::Readable for Op2Spec {}
#[doc = "`write(|w| ..)` method takes [`op2::W`](W) writer structure"]
impl crate::Writable for Op2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OP2 to value 0"]
impl crate::Resettable for Op2Spec {}
