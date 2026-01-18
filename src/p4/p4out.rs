#[doc = "Register `P4OUT` reader"]
pub type R = crate::R<P4outSpec>;
#[doc = "Register `P4OUT` writer"]
pub type W = crate::W<P4outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4outSpec;
impl crate::RegisterSpec for P4outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4out::R`](R) reader structure"]
impl crate::Readable for P4outSpec {}
#[doc = "`write(|w| ..)` method takes [`p4out::W`](W) writer structure"]
impl crate::Writable for P4outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4OUT to value 0"]
impl crate::Resettable for P4outSpec {}
