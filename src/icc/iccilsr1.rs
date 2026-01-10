#[doc = "Register `ICCILSR1` reader"]
pub type R = crate::R<Iccilsr1Spec>;
#[doc = "Register `ICCILSR1` writer"]
pub type W = crate::W<Iccilsr1Spec>;
#[doc = "Field `ILSR8` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr8R = crate::FieldReader;
#[doc = "Field `ILSR8` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR9` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr9R = crate::FieldReader;
#[doc = "Field `ILSR9` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR10` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr10R = crate::FieldReader;
#[doc = "Field `ILSR10` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR11` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type Ilsr11R = crate::FieldReader;
#[doc = "Field `ILSR11` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type Ilsr11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR12` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr12R = crate::FieldReader;
#[doc = "Field `ILSR12` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR13` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr13R = crate::FieldReader;
#[doc = "Field `ILSR13` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR14` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr14R = crate::FieldReader;
#[doc = "Field `ILSR14` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR15` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr15R = crate::FieldReader;
#[doc = "Field `ILSR15` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type Ilsr15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr8(&self) -> Ilsr8R {
        Ilsr8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&self) -> Ilsr9R {
        Ilsr9R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&self) -> Ilsr10R {
        Ilsr10R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&self) -> Ilsr11R {
        Ilsr11R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&self) -> Ilsr12R {
        Ilsr12R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&self) -> Ilsr13R {
        Ilsr13R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&self) -> Ilsr14R {
        Ilsr14R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&self) -> Ilsr15R {
        Ilsr15R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr8(&mut self) -> Ilsr8W<'_, Iccilsr1Spec> {
        Ilsr8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&mut self) -> Ilsr9W<'_, Iccilsr1Spec> {
        Ilsr9W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&mut self) -> Ilsr10W<'_, Iccilsr1Spec> {
        Ilsr10W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&mut self) -> Ilsr11W<'_, Iccilsr1Spec> {
        Ilsr11W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&mut self) -> Ilsr12W<'_, Iccilsr1Spec> {
        Ilsr12W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&mut self) -> Ilsr13W<'_, Iccilsr1Spec> {
        Ilsr13W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&mut self) -> Ilsr14W<'_, Iccilsr1Spec> {
        Ilsr14W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&mut self) -> Ilsr15W<'_, Iccilsr1Spec> {
        Ilsr15W::new(self, 14)
    }
}
#[doc = "ICCILSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccilsr1Spec;
impl crate::RegisterSpec for Iccilsr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccilsr1::R`](R) reader structure"]
impl crate::Readable for Iccilsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iccilsr1::W`](W) writer structure"]
impl crate::Writable for Iccilsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCILSR1 to value 0"]
impl crate::Resettable for Iccilsr1Spec {}
