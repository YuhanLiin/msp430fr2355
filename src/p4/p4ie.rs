#[doc = "Register `P4IE` reader"]
pub type R = crate::R<P4ieSpec>;
#[doc = "Register `P4IE` writer"]
pub type W = crate::W<P4ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4ieSpec;
impl crate::RegisterSpec for P4ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ie::R`](R) reader structure"]
impl crate::Readable for P4ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ie::W`](W) writer structure"]
impl crate::Writable for P4ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4IE to value 0"]
impl crate::Resettable for P4ieSpec {}
