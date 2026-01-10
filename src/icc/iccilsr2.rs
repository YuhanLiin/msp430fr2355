#[doc = "Register `ICCILSR2` reader"]
pub type R = crate::R<Iccilsr2Spec>;
#[doc = "Register `ICCILSR2` writer"]
pub type W = crate::W<Iccilsr2Spec>;
#[doc = "Field `ILSR16` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr16R = crate::FieldReader;
#[doc = "Field `ILSR16` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr16W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR17` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type Ilsr17R = crate::FieldReader;
#[doc = "Field `ILSR17` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type Ilsr17W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR18` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr18R = crate::FieldReader;
#[doc = "Field `ILSR18` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr18W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR19` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr19R = crate::FieldReader;
#[doc = "Field `ILSR19` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr19W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR20` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr20R = crate::FieldReader;
#[doc = "Field `ILSR20` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr20W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR21` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr21R = crate::FieldReader;
#[doc = "Field `ILSR21` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr21W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR22` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type Ilsr22R = crate::FieldReader;
#[doc = "Field `ILSR22` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type Ilsr22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR23` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type Ilsr23R = crate::FieldReader;
#[doc = "Field `ILSR23` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type Ilsr23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&self) -> Ilsr16R {
        Ilsr16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&self) -> Ilsr17R {
        Ilsr17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&self) -> Ilsr18R {
        Ilsr18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&self) -> Ilsr19R {
        Ilsr19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&self) -> Ilsr20R {
        Ilsr20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&self) -> Ilsr21R {
        Ilsr21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&self) -> Ilsr22R {
        Ilsr22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&self) -> Ilsr23R {
        Ilsr23R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&mut self) -> Ilsr16W<'_, Iccilsr2Spec> {
        Ilsr16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&mut self) -> Ilsr17W<'_, Iccilsr2Spec> {
        Ilsr17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&mut self) -> Ilsr18W<'_, Iccilsr2Spec> {
        Ilsr18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&mut self) -> Ilsr19W<'_, Iccilsr2Spec> {
        Ilsr19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&mut self) -> Ilsr20W<'_, Iccilsr2Spec> {
        Ilsr20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&mut self) -> Ilsr21W<'_, Iccilsr2Spec> {
        Ilsr21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&mut self) -> Ilsr22W<'_, Iccilsr2Spec> {
        Ilsr22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&mut self) -> Ilsr23W<'_, Iccilsr2Spec> {
        Ilsr23W::new(self, 14)
    }
}
#[doc = "ICCILSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccilsr2Spec;
impl crate::RegisterSpec for Iccilsr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccilsr2::R`](R) reader structure"]
impl crate::Readable for Iccilsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`iccilsr2::W`](W) writer structure"]
impl crate::Writable for Iccilsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCILSR2 to value 0"]
impl crate::Resettable for Iccilsr2Spec {}
