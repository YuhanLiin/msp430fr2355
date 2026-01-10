#[doc = "Register `ICCILSR3` reader"]
pub type R = crate::R<Iccilsr3Spec>;
#[doc = "Register `ICCILSR3` writer"]
pub type W = crate::W<Iccilsr3Spec>;
#[doc = "Field `ILSR24` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr24R = crate::FieldReader;
#[doc = "Field `ILSR24` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr24W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR25` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr25R = crate::FieldReader;
#[doc = "Field `ILSR25` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr25W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR26` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr26R = crate::FieldReader;
#[doc = "Field `ILSR26` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr26W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR27` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr27R = crate::FieldReader;
#[doc = "Field `ILSR27` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr27W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR28` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr28R = crate::FieldReader;
#[doc = "Field `ILSR28` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr28W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR29` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr29R = crate::FieldReader;
#[doc = "Field `ILSR29` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr29W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR30` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr30R = crate::FieldReader;
#[doc = "Field `ILSR30` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR31` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr31R = crate::FieldReader;
#[doc = "Field `ILSR31` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr31W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr24(&self) -> Ilsr24R {
        Ilsr24R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&self) -> Ilsr25R {
        Ilsr25R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&self) -> Ilsr26R {
        Ilsr26R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&self) -> Ilsr27R {
        Ilsr27R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&self) -> Ilsr28R {
        Ilsr28R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&self) -> Ilsr29R {
        Ilsr29R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&self) -> Ilsr30R {
        Ilsr30R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&self) -> Ilsr31R {
        Ilsr31R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr24(&mut self) -> Ilsr24W<'_, Iccilsr3Spec> {
        Ilsr24W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&mut self) -> Ilsr25W<'_, Iccilsr3Spec> {
        Ilsr25W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&mut self) -> Ilsr26W<'_, Iccilsr3Spec> {
        Ilsr26W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&mut self) -> Ilsr27W<'_, Iccilsr3Spec> {
        Ilsr27W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&mut self) -> Ilsr28W<'_, Iccilsr3Spec> {
        Ilsr28W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&mut self) -> Ilsr29W<'_, Iccilsr3Spec> {
        Ilsr29W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&mut self) -> Ilsr30W<'_, Iccilsr3Spec> {
        Ilsr30W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&mut self) -> Ilsr31W<'_, Iccilsr3Spec> {
        Ilsr31W::new(self, 14)
    }
}
#[doc = "ICCILSR3\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccilsr3Spec;
impl crate::RegisterSpec for Iccilsr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccilsr3::R`](R) reader structure"]
impl crate::Readable for Iccilsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`iccilsr3::W`](W) writer structure"]
impl crate::Writable for Iccilsr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCILSR3 to value 0"]
impl crate::Resettable for Iccilsr3Spec {}
