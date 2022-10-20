#[doc = "Register `SAC2DAC` reader"]
pub struct R(crate::R<SAC2DAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC2DAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC2DAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC2DAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC2DAC` writer"]
pub struct W(crate::W<SAC2DAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC2DAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAC2DAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC2DAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACEN` reader - SAC DAC enable"]
pub type DACEN_R = crate::BitReader<DACEN_A>;
#[doc = "SAC DAC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACEN_A {
    #[doc = "0: Disabled"]
    DACEN_0 = 0,
    #[doc = "1: Enabled"]
    DACEN_1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DACEN_0,
            true => DACEN_A::DACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACEN_0`"]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        *self == DACEN_A::DACEN_0
    }
    #[doc = "Checks if the value of the field is `DACEN_1`"]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        *self == DACEN_A::DACEN_1
    }
}
#[doc = "Field `DACEN` writer - SAC DAC enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC2DAC_SPEC, DACEN_A, O>;
impl<'a, const O: u8> DACEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_1)
    }
}
#[doc = "Field `DACIE` reader - SAC DAC interrupt enable"]
pub type DACIE_R = crate::BitReader<DACIE_A>;
#[doc = "SAC DAC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACIE_A {
    #[doc = "0: Disabled"]
    DACIE_0 = 0,
    #[doc = "1: Enabled"]
    DACIE_1 = 1,
}
impl From<DACIE_A> for bool {
    #[inline(always)]
    fn from(variant: DACIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DACIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACIE_A {
        match self.bits {
            false => DACIE_A::DACIE_0,
            true => DACIE_A::DACIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACIE_0`"]
    #[inline(always)]
    pub fn is_dacie_0(&self) -> bool {
        *self == DACIE_A::DACIE_0
    }
    #[doc = "Checks if the value of the field is `DACIE_1`"]
    #[inline(always)]
    pub fn is_dacie_1(&self) -> bool {
        *self == DACIE_A::DACIE_1
    }
}
#[doc = "Field `DACIE` writer - SAC DAC interrupt enable"]
pub type DACIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC2DAC_SPEC, DACIE_A, O>;
impl<'a, const O: u8> DACIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacie_0(self) -> &'a mut W {
        self.variant(DACIE_A::DACIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacie_1(self) -> &'a mut W {
        self.variant(DACIE_A::DACIE_1)
    }
}
#[doc = "Field `DACDMAE` reader - SAC DAC DMA request enable"]
pub type DACDMAE_R = crate::BitReader<DACDMAE_A>;
#[doc = "SAC DAC DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACDMAE_A {
    #[doc = "0: DMA request disabled"]
    DACDMAE_0 = 0,
    #[doc = "1: DMA request enabled"]
    DACDMAE_1 = 1,
}
impl From<DACDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DACDMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl DACDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACDMAE_A {
        match self.bits {
            false => DACDMAE_A::DACDMAE_0,
            true => DACDMAE_A::DACDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACDMAE_0`"]
    #[inline(always)]
    pub fn is_dacdmae_0(&self) -> bool {
        *self == DACDMAE_A::DACDMAE_0
    }
    #[doc = "Checks if the value of the field is `DACDMAE_1`"]
    #[inline(always)]
    pub fn is_dacdmae_1(&self) -> bool {
        *self == DACDMAE_A::DACDMAE_1
    }
}
#[doc = "Field `DACDMAE` writer - SAC DAC DMA request enable"]
pub type DACDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC2DAC_SPEC, DACDMAE_A, O>;
impl<'a, const O: u8> DACDMAE_W<'a, O> {
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn dacdmae_0(self) -> &'a mut W {
        self.variant(DACDMAE_A::DACDMAE_0)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn dacdmae_1(self) -> &'a mut W {
        self.variant(DACDMAE_A::DACDMAE_1)
    }
}
#[doc = "Field `DACLSEL` reader - SAC DAC load select. Selects the load trigger for the DAC latch."]
pub type DACLSEL_R = crate::FieldReader<u8, DACLSEL_A>;
#[doc = "SAC DAC load select. Selects the load trigger for the DAC latch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACLSEL_A {
    #[doc = "0: DAC latch loads when DACDAT written"]
    DACLSEL_0 = 0,
    #[doc = "2: Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    DACLSEL_2 = 2,
    #[doc = "3: Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    DACLSEL_3 = 3,
}
impl From<DACLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DACLSEL_A) -> Self {
        variant as _
    }
}
impl DACLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DACLSEL_A> {
        match self.bits {
            0 => Some(DACLSEL_A::DACLSEL_0),
            2 => Some(DACLSEL_A::DACLSEL_2),
            3 => Some(DACLSEL_A::DACLSEL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DACLSEL_0`"]
    #[inline(always)]
    pub fn is_daclsel_0(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_0
    }
    #[doc = "Checks if the value of the field is `DACLSEL_2`"]
    #[inline(always)]
    pub fn is_daclsel_2(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_2
    }
    #[doc = "Checks if the value of the field is `DACLSEL_3`"]
    #[inline(always)]
    pub fn is_daclsel_3(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_3
    }
}
#[doc = "Field `DACLSEL` writer - SAC DAC load select. Selects the load trigger for the DAC latch."]
pub type DACLSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SAC2DAC_SPEC, u8, DACLSEL_A, 2, O>;
impl<'a, const O: u8> DACLSEL_W<'a, O> {
    #[doc = "DAC latch loads when DACDAT written"]
    #[inline(always)]
    pub fn daclsel_0(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_0)
    }
    #[doc = "Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_2(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_2)
    }
    #[doc = "Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_3(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_3)
    }
}
#[doc = "Field `DACSREF` reader - SAC DAC select reference voltage"]
pub type DACSREF_R = crate::BitReader<DACSREF_A>;
#[doc = "SAC DAC select reference voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACSREF_A {
    #[doc = "0: AVCC"]
    DACSREF_0 = 0,
    #[doc = "1: Alternative reference"]
    DACSREF_1 = 1,
}
impl From<DACSREF_A> for bool {
    #[inline(always)]
    fn from(variant: DACSREF_A) -> Self {
        variant as u8 != 0
    }
}
impl DACSREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSREF_A {
        match self.bits {
            false => DACSREF_A::DACSREF_0,
            true => DACSREF_A::DACSREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACSREF_0`"]
    #[inline(always)]
    pub fn is_dacsref_0(&self) -> bool {
        *self == DACSREF_A::DACSREF_0
    }
    #[doc = "Checks if the value of the field is `DACSREF_1`"]
    #[inline(always)]
    pub fn is_dacsref_1(&self) -> bool {
        *self == DACSREF_A::DACSREF_1
    }
}
#[doc = "Field `DACSREF` writer - SAC DAC select reference voltage"]
pub type DACSREF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC2DAC_SPEC, DACSREF_A, O>;
impl<'a, const O: u8> DACSREF_W<'a, O> {
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn dacsref_0(self) -> &'a mut W {
        self.variant(DACSREF_A::DACSREF_0)
    }
    #[doc = "Alternative reference"]
    #[inline(always)]
    pub fn dacsref_1(self) -> &'a mut W {
        self.variant(DACSREF_A::DACSREF_1)
    }
}
impl R {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&self) -> DACIE_R {
        DACIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&self) -> DACDMAE_R {
        DACDMAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&self) -> DACLSEL_R {
        DACLSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&self) -> DACSREF_R {
        DACSREF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<0> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&mut self) -> DACIE_W<1> {
        DACIE_W::new(self)
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&mut self) -> DACDMAE_W<2> {
        DACDMAE_W::new(self)
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&mut self) -> DACLSEL_W<8> {
        DACLSEL_W::new(self)
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&mut self) -> DACSREF_W<12> {
        DACSREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAC DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2dac](index.html) module"]
pub struct SAC2DAC_SPEC;
impl crate::RegisterSpec for SAC2DAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac2dac::R](R) reader structure"]
impl crate::Readable for SAC2DAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac2dac::W](W) writer structure"]
impl crate::Writable for SAC2DAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC2DAC to value 0"]
impl crate::Resettable for SAC2DAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
