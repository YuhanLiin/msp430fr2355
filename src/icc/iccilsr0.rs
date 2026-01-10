#[doc = "Register `ICCILSR0` reader"]
pub type R = crate::R<Iccilsr0Spec>;
#[doc = "Register `ICCILSR0` writer"]
pub type W = crate::W<Iccilsr0Spec>;
#[doc = "Field `ILSR0` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr0R = crate::FieldReader;
#[doc = "Field `ILSR0` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR1` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr1R = crate::FieldReader;
#[doc = "Field `ILSR1` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR2` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr2R = crate::FieldReader;
#[doc = "Field `ILSR2` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR3` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr3R = crate::FieldReader;
#[doc = "Field `ILSR3` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR4` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr4R = crate::FieldReader;
#[doc = "Field `ILSR4` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR5` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr5R = crate::FieldReader;
#[doc = "Field `ILSR5` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR6` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr6R = crate::FieldReader;
#[doc = "Field `ILSR6` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ILSR7` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr7R = crate::FieldReader;
#[doc = "Field `ILSR7` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type Ilsr7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&self) -> Ilsr0R {
        Ilsr0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&self) -> Ilsr1R {
        Ilsr1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&self) -> Ilsr2R {
        Ilsr2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&self) -> Ilsr3R {
        Ilsr3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&self) -> Ilsr4R {
        Ilsr4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&self) -> Ilsr5R {
        Ilsr5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&self) -> Ilsr6R {
        Ilsr6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&self) -> Ilsr7R {
        Ilsr7R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&mut self) -> Ilsr0W<'_, Iccilsr0Spec> {
        Ilsr0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&mut self) -> Ilsr1W<'_, Iccilsr0Spec> {
        Ilsr1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&mut self) -> Ilsr2W<'_, Iccilsr0Spec> {
        Ilsr2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&mut self) -> Ilsr3W<'_, Iccilsr0Spec> {
        Ilsr3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&mut self) -> Ilsr4W<'_, Iccilsr0Spec> {
        Ilsr4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&mut self) -> Ilsr5W<'_, Iccilsr0Spec> {
        Ilsr5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&mut self) -> Ilsr6W<'_, Iccilsr0Spec> {
        Ilsr6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&mut self) -> Ilsr7W<'_, Iccilsr0Spec> {
        Ilsr7W::new(self, 14)
    }
}
#[doc = "ICCILSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccilsr0Spec;
impl crate::RegisterSpec for Iccilsr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccilsr0::R`](R) reader structure"]
impl crate::Readable for Iccilsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`iccilsr0::W`](W) writer structure"]
impl crate::Writable for Iccilsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCILSR0 to value 0"]
impl crate::Resettable for Iccilsr0Spec {}
