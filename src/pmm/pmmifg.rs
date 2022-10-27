#[doc = "Register `PMMIFG` reader"]
pub struct R(crate::R<PMMIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMIFG` writer"]
pub struct W(crate::W<PMMIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMIFG_SPEC>;
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
impl From<crate::W<PMMIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMSPSIFG` reader - PMM secondary power supply interrupt flag. Reserved for future multi power supply systems."]
pub type PMMSPSIFG_R = crate::BitReader<bool>;
#[doc = "Field `PMMBORIFG` reader - PMM software brownout reset interrupt flag."]
pub type PMMBORIFG_R = crate::BitReader<PMMBORIFG_A>;
#[doc = "PMM software brownout reset interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMBORIFG_A {
    #[doc = "0: Reset not due to PMMSWBOR"]
    PMMBORIFG_0 = 0,
    #[doc = "1: Reset due to PMMSWBOR"]
    PMMBORIFG_1 = 1,
}
impl From<PMMBORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMBORIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMBORIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMBORIFG_A {
        match self.bits {
            false => PMMBORIFG_A::PMMBORIFG_0,
            true => PMMBORIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMBORIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMBORIFG_A::PMMBORIFG_1
    }
}
#[doc = "Field `PMMBORIFG` writer - PMM software brownout reset interrupt flag."]
pub type PMMBORIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, PMMBORIFG_A, O>;
impl<'a, const O: u8> PMMBORIFG_W<'a, O> {
    #[doc = "Reset not due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMBORIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMBORIFG_A::PMMBORIFG_1)
    }
}
#[doc = "Field `PMMRSTIFG` reader - PMM reset pin interrupt flag."]
pub type PMMRSTIFG_R = crate::BitReader<PMMRSTIFG_A>;
#[doc = "PMM reset pin interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMRSTIFG_A {
    #[doc = "0: Reset not due to reset pin"]
    PMMBORIFG_0 = 0,
    #[doc = "1: Reset due to reset pin"]
    PMMBORIFG_1 = 1,
}
impl From<PMMRSTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMRSTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMRSTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMRSTIFG_A {
        match self.bits {
            false => PMMRSTIFG_A::PMMBORIFG_0,
            true => PMMRSTIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMRSTIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMRSTIFG_A::PMMBORIFG_1
    }
}
#[doc = "Field `PMMRSTIFG` writer - PMM reset pin interrupt flag."]
pub type PMMRSTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, PMMRSTIFG_A, O>;
impl<'a, const O: u8> PMMRSTIFG_W<'a, O> {
    #[doc = "Reset not due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMRSTIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMRSTIFG_A::PMMBORIFG_1)
    }
}
#[doc = "Field `PMMPORIFG` reader - PMM software POR interrupt flag."]
pub type PMMPORIFG_R = crate::BitReader<PMMPORIFG_A>;
#[doc = "PMM software POR interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMPORIFG_A {
    #[doc = "0: Reset not due to PMMSWPOR"]
    PMMBORIFG_0 = 0,
    #[doc = "1: Reset due to PMMSWPOR"]
    PMMBORIFG_1 = 1,
}
impl From<PMMPORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMPORIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMPORIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMPORIFG_A {
        match self.bits {
            false => PMMPORIFG_A::PMMBORIFG_0,
            true => PMMPORIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMPORIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMPORIFG_A::PMMBORIFG_1
    }
}
#[doc = "Field `PMMPORIFG` writer - PMM software POR interrupt flag."]
pub type PMMPORIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, PMMPORIFG_A, O>;
impl<'a, const O: u8> PMMPORIFG_W<'a, O> {
    #[doc = "Reset not due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMPORIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMPORIFG_A::PMMBORIFG_1)
    }
}
#[doc = "Field `SPWRIFG` reader - Secondary Power interrupt flag. This bit only works in multi power supply systems. When the secondary power is ready to use, this bit is set., In single power supply systems, this bit does not work."]
pub type SPWRIFG_R = crate::BitReader<bool>;
#[doc = "Field `PPWRIFG` reader - Primary Power interrupt flag. This bit only works in multi power supply systems. When the primary power is ready to use, this bit is set. In single power supply systems, this bit does not work"]
pub type PPWRIFG_R = crate::BitReader<bool>;
#[doc = "Field `SVSHIFG` reader - High-side SVS interrupt flag."]
pub type SVSHIFG_R = crate::BitReader<SVSHIFG_A>;
#[doc = "High-side SVS interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVSHIFG_A {
    #[doc = "0: Reset not due to SVSH"]
    SVSHIFG_0 = 0,
    #[doc = "1: Reset due to SVSH"]
    SVSHIFG_1 = 1,
}
impl From<SVSHIFG_A> for bool {
    #[inline(always)]
    fn from(variant: SVSHIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SVSHIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHIFG_A {
        match self.bits {
            false => SVSHIFG_A::SVSHIFG_0,
            true => SVSHIFG_A::SVSHIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSHIFG_0`"]
    #[inline(always)]
    pub fn is_svshifg_0(&self) -> bool {
        *self == SVSHIFG_A::SVSHIFG_0
    }
    #[doc = "Checks if the value of the field is `SVSHIFG_1`"]
    #[inline(always)]
    pub fn is_svshifg_1(&self) -> bool {
        *self == SVSHIFG_A::SVSHIFG_1
    }
}
#[doc = "Field `SVSHIFG` writer - High-side SVS interrupt flag."]
pub type SVSHIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, SVSHIFG_A, O>;
impl<'a, const O: u8> SVSHIFG_W<'a, O> {
    #[doc = "Reset not due to SVSH"]
    #[inline(always)]
    pub fn svshifg_0(self) -> &'a mut W {
        self.variant(SVSHIFG_A::SVSHIFG_0)
    }
    #[doc = "Reset due to SVSH"]
    #[inline(always)]
    pub fn svshifg_1(self) -> &'a mut W {
        self.variant(SVSHIFG_A::SVSHIFG_1)
    }
}
#[doc = "Field `PMMLPM5IFG` reader - LPMx.5 flag."]
pub type PMMLPM5IFG_R = crate::BitReader<PMMLPM5IFG_A>;
#[doc = "LPMx.5 flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMLPM5IFG_A {
    #[doc = "0: Reset not due to wake-up from LPMx.5"]
    PMMLPM5IFG_0 = 0,
    #[doc = "1: Reset due to wake-up from LPMx.5"]
    PMMLPM5IFG_1 = 1,
}
impl From<PMMLPM5IFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMLPM5IFG_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMLPM5IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMLPM5IFG_A {
        match self.bits {
            false => PMMLPM5IFG_A::PMMLPM5IFG_0,
            true => PMMLPM5IFG_A::PMMLPM5IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMLPM5IFG_0`"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_0(&self) -> bool {
        *self == PMMLPM5IFG_A::PMMLPM5IFG_0
    }
    #[doc = "Checks if the value of the field is `PMMLPM5IFG_1`"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_1(&self) -> bool {
        *self == PMMLPM5IFG_A::PMMLPM5IFG_1
    }
}
#[doc = "Field `PMMLPM5IFG` writer - LPMx.5 flag."]
pub type PMMLPM5IFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, PMMLPM5IFG_A, O>;
impl<'a, const O: u8> PMMLPM5IFG_W<'a, O> {
    #[doc = "Reset not due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_0(self) -> &'a mut W {
        self.variant(PMMLPM5IFG_A::PMMLPM5IFG_0)
    }
    #[doc = "Reset due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_1(self) -> &'a mut W {
        self.variant(PMMLPM5IFG_A::PMMLPM5IFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - PMM secondary power supply interrupt flag. Reserved for future multi power supply systems."]
    #[inline(always)]
    pub fn pmmspsifg(&self) -> PMMSPSIFG_R {
        PMMSPSIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - PMM software brownout reset interrupt flag."]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PMMBORIFG_R {
        PMMBORIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMM reset pin interrupt flag."]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PMMRSTIFG_R {
        PMMRSTIFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMM software POR interrupt flag."]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PMMPORIFG_R {
        PMMPORIFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secondary Power interrupt flag. This bit only works in multi power supply systems. When the secondary power is ready to use, this bit is set., In single power supply systems, this bit does not work."]
    #[inline(always)]
    pub fn spwrifg(&self) -> SPWRIFG_R {
        SPWRIFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Primary Power interrupt flag. This bit only works in multi power supply systems. When the primary power is ready to use, this bit is set. In single power supply systems, this bit does not work"]
    #[inline(always)]
    pub fn ppwrifg(&self) -> PPWRIFG_R {
        PPWRIFG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - High-side SVS interrupt flag."]
    #[inline(always)]
    pub fn svshifg(&self) -> SVSHIFG_R {
        SVSHIFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - LPMx.5 flag."]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> PMMLPM5IFG_R {
        PMMLPM5IFG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PMM software brownout reset interrupt flag."]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PMMBORIFG_W<8> {
        PMMBORIFG_W::new(self)
    }
    #[doc = "Bit 9 - PMM reset pin interrupt flag."]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PMMRSTIFG_W<9> {
        PMMRSTIFG_W::new(self)
    }
    #[doc = "Bit 10 - PMM software POR interrupt flag."]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PMMPORIFG_W<10> {
        PMMPORIFG_W::new(self)
    }
    #[doc = "Bit 13 - High-side SVS interrupt flag."]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SVSHIFG_W<13> {
        SVSHIFG_W::new(self)
    }
    #[doc = "Bit 15 - LPMx.5 flag."]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> PMMLPM5IFG_W<15> {
        PMMLPM5IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](index.html) module"]
pub struct PMMIFG_SPEC;
impl crate::RegisterSpec for PMMIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmifg::R](R) reader structure"]
impl crate::Readable for PMMIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](W) writer structure"]
impl crate::Writable for PMMIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMIFG to value 0"]
impl crate::Resettable for PMMIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
