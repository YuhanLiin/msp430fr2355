#[doc = "Register `P2DIR` reader"]
pub type R = crate::R<P2dirSpec>;
#[doc = "Register `P2DIR` writer"]
pub type W = crate::W<P2dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p2dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2dirSpec;
impl crate::RegisterSpec for P2dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2dir::R`](R) reader structure"]
impl crate::Readable for P2dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p2dir::W`](W) writer structure"]
impl crate::Writable for P2dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2DIR to value 0"]
impl crate::Resettable for P2dirSpec {}
