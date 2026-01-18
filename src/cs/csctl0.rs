#[doc = "Register `CSCTL0` reader"]
pub type R = crate::R<Csctl0Spec>;
#[doc = "Register `CSCTL0` writer"]
pub type W = crate::W<Csctl0Spec>;
#[doc = "Field `DCO` reader - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
pub type DcoR = crate::FieldReader<u16>;
#[doc = "Field `DCO` writer - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
pub type DcoW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MOD` reader - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
pub type ModR = crate::FieldReader;
#[doc = "Field `MOD` writer - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&self) -> DcoR {
        DcoR::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&mut self) -> DcoW<'_, Csctl0Spec> {
        DcoW::new(self, 0)
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, Csctl0Spec> {
        ModW::new(self, 9)
    }
}
#[doc = "Clock System Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl0Spec;
impl crate::RegisterSpec for Csctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl0::R`](R) reader structure"]
impl crate::Readable for Csctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl0::W`](W) writer structure"]
impl crate::Writable for Csctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for Csctl0Spec {}
