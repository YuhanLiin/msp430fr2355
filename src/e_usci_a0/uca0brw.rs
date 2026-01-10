#[doc = "Register `UCA0BRW` reader"]
pub type R = crate::R<Uca0brwSpec>;
#[doc = "Register `UCA0BRW` writer"]
pub type W = crate::W<Uca0brwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0brwSpec;
impl crate::RegisterSpec for Uca0brwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0brw::R`](R) reader structure"]
impl crate::Readable for Uca0brwSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0brw::W`](W) writer structure"]
impl crate::Writable for Uca0brwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0BRW to value 0"]
impl crate::Resettable for Uca0brwSpec {}
