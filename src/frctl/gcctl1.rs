#[doc = "Register `GCCTL1` reader"]
pub type R = crate::R<Gcctl1Spec>;
#[doc = "Register `GCCTL1` writer"]
pub type W = crate::W<Gcctl1Spec>;
#[doc = "FRAM correctable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbdifg {
    #[doc = "0: No interrupt is pending"]
    Cbdifg0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    Cbdifg1 = 1,
}
impl From<Cbdifg> for bool {
    #[inline(always)]
    fn from(variant: Cbdifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error detection flag"]
pub type CbdifgR = crate::BitReader<Cbdifg>;
impl CbdifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbdifg {
        match self.bits {
            false => Cbdifg::Cbdifg0,
            true => Cbdifg::Cbdifg1,
        }
    }
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn is_cbdifg_0(&self) -> bool {
        *self == Cbdifg::Cbdifg0
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    #[inline(always)]
    pub fn is_cbdifg_1(&self) -> bool {
        *self == Cbdifg::Cbdifg1
    }
}
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error detection flag"]
pub type CbdifgW<'a, REG> = crate::BitWriter<'a, REG, Cbdifg>;
impl<'a, REG> CbdifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn cbdifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdifg::Cbdifg0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    #[inline(always)]
    pub fn cbdifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cbdifg::Cbdifg1)
    }
}
#[doc = "FRAM uncorrectable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ubdifg {
    #[doc = "0: No interrupt pending."]
    Ubdifg0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    Ubdifg1 = 1,
}
impl From<Ubdifg> for bool {
    #[inline(always)]
    fn from(variant: Ubdifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error detection flag"]
pub type UbdifgR = crate::BitReader<Ubdifg>;
impl UbdifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ubdifg {
        match self.bits {
            false => Ubdifg::Ubdifg0,
            true => Ubdifg::Ubdifg1,
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn is_ubdifg_0(&self) -> bool {
        *self == Ubdifg::Ubdifg0
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn is_ubdifg_1(&self) -> bool {
        *self == Ubdifg::Ubdifg1
    }
}
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error detection flag"]
pub type UbdifgW<'a, REG> = crate::BitWriter<'a, REG, Ubdifg>;
impl<'a, REG> UbdifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn ubdifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdifg::Ubdifg0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn ubdifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ubdifg::Ubdifg1)
    }
}
#[doc = "Access time error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accteifg {
    #[doc = "0: No interrupt pending."]
    Accteifg0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    Accteifg1 = 1,
}
impl From<Accteifg> for bool {
    #[inline(always)]
    fn from(variant: Accteifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub type AccteifgR = crate::BitReader<Accteifg>;
impl AccteifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accteifg {
        match self.bits {
            false => Accteifg::Accteifg0,
            true => Accteifg::Accteifg1,
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn is_accteifg_0(&self) -> bool {
        *self == Accteifg::Accteifg0
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn is_accteifg_1(&self) -> bool {
        *self == Accteifg::Accteifg1
    }
}
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub type AccteifgW<'a, REG> = crate::BitWriter<'a, REG, Accteifg>;
impl<'a, REG> AccteifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn accteifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Accteifg::Accteifg0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn accteifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Accteifg::Accteifg1)
    }
}
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CbdifgR {
        CbdifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UbdifgR {
        UbdifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> AccteifgR {
        AccteifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CbdifgW<'_, Gcctl1Spec> {
        CbdifgW::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UbdifgW<'_, Gcctl1Spec> {
        UbdifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> AccteifgW<'_, Gcctl1Spec> {
        AccteifgW::new(self, 3)
    }
}
#[doc = "General Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcctl1Spec;
impl crate::RegisterSpec for Gcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl1::R`](R) reader structure"]
impl crate::Readable for Gcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`gcctl1::W`](W) writer structure"]
impl crate::Writable for Gcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for Gcctl1Spec {}
