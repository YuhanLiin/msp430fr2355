#[doc = "Register `PMMCTL2` reader"]
pub struct R(crate::R<PMMCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL2` writer"]
pub struct W(crate::W<PMMCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL2_SPEC>;
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
impl From<crate::W<PMMCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTREFEN` reader - Internal reference enable"]
pub type INTREFEN_R = crate::BitReader<INTREFEN_A>;
#[doc = "Internal reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTREFEN_A {
    #[doc = "0: Disable internal reference"]
    INTREFEN_0 = 0,
    #[doc = "1: Enable internal reference"]
    INTREFEN_1 = 1,
}
impl From<INTREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTREFEN_A {
        match self.bits {
            false => INTREFEN_A::INTREFEN_0,
            true => INTREFEN_A::INTREFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTREFEN_0`"]
    #[inline(always)]
    pub fn is_intrefen_0(&self) -> bool {
        *self == INTREFEN_A::INTREFEN_0
    }
    #[doc = "Checks if the value of the field is `INTREFEN_1`"]
    #[inline(always)]
    pub fn is_intrefen_1(&self) -> bool {
        *self == INTREFEN_A::INTREFEN_1
    }
}
#[doc = "Field `INTREFEN` writer - Internal reference enable"]
pub type INTREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, INTREFEN_A, O>;
impl<'a, const O: u8> INTREFEN_W<'a, O> {
    #[doc = "Disable internal reference"]
    #[inline(always)]
    pub fn intrefen_0(self) -> &'a mut W {
        self.variant(INTREFEN_A::INTREFEN_0)
    }
    #[doc = "Enable internal reference"]
    #[inline(always)]
    pub fn intrefen_1(self) -> &'a mut W {
        self.variant(INTREFEN_A::INTREFEN_1)
    }
}
#[doc = "Field `EXTREFEN` reader - External reference output enable"]
pub type EXTREFEN_R = crate::BitReader<EXTREFEN_A>;
#[doc = "External reference output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTREFEN_A {
    #[doc = "0: Disable external reference output"]
    EXTREFEN_0 = 0,
    #[doc = "1: Enable internal reference output"]
    EXTREFEN_1 = 1,
}
impl From<EXTREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTREFEN_A {
        match self.bits {
            false => EXTREFEN_A::EXTREFEN_0,
            true => EXTREFEN_A::EXTREFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXTREFEN_0`"]
    #[inline(always)]
    pub fn is_extrefen_0(&self) -> bool {
        *self == EXTREFEN_A::EXTREFEN_0
    }
    #[doc = "Checks if the value of the field is `EXTREFEN_1`"]
    #[inline(always)]
    pub fn is_extrefen_1(&self) -> bool {
        *self == EXTREFEN_A::EXTREFEN_1
    }
}
#[doc = "Field `EXTREFEN` writer - External reference output enable"]
pub type EXTREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, EXTREFEN_A, O>;
impl<'a, const O: u8> EXTREFEN_W<'a, O> {
    #[doc = "Disable external reference output"]
    #[inline(always)]
    pub fn extrefen_0(self) -> &'a mut W {
        self.variant(EXTREFEN_A::EXTREFEN_0)
    }
    #[doc = "Enable internal reference output"]
    #[inline(always)]
    pub fn extrefen_1(self) -> &'a mut W {
        self.variant(EXTREFEN_A::EXTREFEN_1)
    }
}
#[doc = "Field `TSENSOREN` reader - Temperature sensor enable"]
pub type TSENSOREN_R = crate::BitReader<TSENSOREN_A>;
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSENSOREN_A {
    #[doc = "0: Disable temperature sensor"]
    TSENSOREN_0 = 0,
    #[doc = "1: Enable temperature sensor"]
    TSENSOREN_1 = 1,
}
impl From<TSENSOREN_A> for bool {
    #[inline(always)]
    fn from(variant: TSENSOREN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSENSOREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSENSOREN_A {
        match self.bits {
            false => TSENSOREN_A::TSENSOREN_0,
            true => TSENSOREN_A::TSENSOREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSENSOREN_0`"]
    #[inline(always)]
    pub fn is_tsensoren_0(&self) -> bool {
        *self == TSENSOREN_A::TSENSOREN_0
    }
    #[doc = "Checks if the value of the field is `TSENSOREN_1`"]
    #[inline(always)]
    pub fn is_tsensoren_1(&self) -> bool {
        *self == TSENSOREN_A::TSENSOREN_1
    }
}
#[doc = "Field `TSENSOREN` writer - Temperature sensor enable"]
pub type TSENSOREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, TSENSOREN_A, O>;
impl<'a, const O: u8> TSENSOREN_W<'a, O> {
    #[doc = "Disable temperature sensor"]
    #[inline(always)]
    pub fn tsensoren_0(self) -> &'a mut W {
        self.variant(TSENSOREN_A::TSENSOREN_0)
    }
    #[doc = "Enable temperature sensor"]
    #[inline(always)]
    pub fn tsensoren_1(self) -> &'a mut W {
        self.variant(TSENSOREN_A::TSENSOREN_1)
    }
}
#[doc = "Field `REFVSEL` reader - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
pub type REFVSEL_R = crate::FieldReader<u8, REFVSEL_A>;
#[doc = "Reference voltage level select. Can be modified only when REFGENBUSY = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: 00b = 1.5V"]
    REFVSEL_0 = 0,
    #[doc = "1: 01b = 2.0V"]
    REFVSEL_1 = 1,
    #[doc = "2: 10b = 2.5V"]
    REFVSEL_2 = 2,
    #[doc = "3: 11b = Reserved"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
impl REFVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Field `REFVSEL` writer - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
pub type REFVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PMMCTL2_SPEC, u8, REFVSEL_A, 2, O>;
impl<'a, const O: u8> REFVSEL_W<'a, O> {
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
}
#[doc = "Field `REFGEN` reader - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
pub type REFGEN_R = crate::BitReader<REFGEN_A>;
#[doc = "Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGEN_A {
    #[doc = "0: No trigger"]
    REFGEN_0 = 0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    REFGEN_1 = 1,
}
impl From<REFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REFGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGEN_A {
        match self.bits {
            false => REFGEN_A::REFGEN_0,
            true => REFGEN_A::REFGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGEN_0`"]
    #[inline(always)]
    pub fn is_refgen_0(&self) -> bool {
        *self == REFGEN_A::REFGEN_0
    }
    #[doc = "Checks if the value of the field is `REFGEN_1`"]
    #[inline(always)]
    pub fn is_refgen_1(&self) -> bool {
        *self == REFGEN_A::REFGEN_1
    }
}
#[doc = "Field `REFGEN` writer - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
pub type REFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, REFGEN_A, O>;
impl<'a, const O: u8> REFGEN_W<'a, O> {
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refgen_0(self) -> &'a mut W {
        self.variant(REFGEN_A::REFGEN_0)
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refgen_1(self) -> &'a mut W {
        self.variant(REFGEN_A::REFGEN_1)
    }
}
#[doc = "Field `REFBGEN` reader - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
pub type REFBGEN_R = crate::BitReader<REFBGEN_A>;
#[doc = "Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGEN_A {
    #[doc = "0: No trigger"]
    REFBG_0 = 0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    REFBG_1 = 1,
}
impl From<REFBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGEN_A {
        match self.bits {
            false => REFBGEN_A::REFBG_0,
            true => REFBGEN_A::REFBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBG_0`"]
    #[inline(always)]
    pub fn is_refbg_0(&self) -> bool {
        *self == REFBGEN_A::REFBG_0
    }
    #[doc = "Checks if the value of the field is `REFBG_1`"]
    #[inline(always)]
    pub fn is_refbg_1(&self) -> bool {
        *self == REFBGEN_A::REFBG_1
    }
}
#[doc = "Field `REFBGEN` writer - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
pub type REFBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, REFBGEN_A, O>;
impl<'a, const O: u8> REFBGEN_W<'a, O> {
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refbg_0(self) -> &'a mut W {
        self.variant(REFBGEN_A::REFBG_0)
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refbg_1(self) -> &'a mut W {
        self.variant(REFBGEN_A::REFBG_1)
    }
}
#[doc = "Field `REFGENACT` reader - Reference generator active. Read only."]
pub type REFGENACT_R = crate::BitReader<REFGENACT_A>;
#[doc = "Reference generator active. Read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENACT_A {
    #[doc = "0: Reference generator not active"]
    REFGENACT_0 = 0,
    #[doc = "1: Reference generator active"]
    REFGENACT_1 = 1,
}
impl From<REFGENACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENACT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENACT_A {
        match self.bits {
            false => REFGENACT_A::REFGENACT_0,
            true => REFGENACT_A::REFGENACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENACT_0`"]
    #[inline(always)]
    pub fn is_refgenact_0(&self) -> bool {
        *self == REFGENACT_A::REFGENACT_0
    }
    #[doc = "Checks if the value of the field is `REFGENACT_1`"]
    #[inline(always)]
    pub fn is_refgenact_1(&self) -> bool {
        *self == REFGENACT_A::REFGENACT_1
    }
}
#[doc = "Field `REFBGACT` reader - Reference bandgap active. Ready only."]
pub type REFBGACT_R = crate::BitReader<REFBGACT_A>;
#[doc = "Reference bandgap active. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGACT_A {
    #[doc = "0: Reference bandgap buffer not active"]
    REFBGACT_0 = 0,
    #[doc = "1: Reference bandgap buffer active"]
    REFBGACT_1 = 1,
}
impl From<REFBGACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGACT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGACT_A {
        match self.bits {
            false => REFBGACT_A::REFBGACT_0,
            true => REFBGACT_A::REFBGACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGACT_0`"]
    #[inline(always)]
    pub fn is_refbgact_0(&self) -> bool {
        *self == REFBGACT_A::REFBGACT_0
    }
    #[doc = "Checks if the value of the field is `REFBGACT_1`"]
    #[inline(always)]
    pub fn is_refbgact_1(&self) -> bool {
        *self == REFBGACT_A::REFBGACT_1
    }
}
#[doc = "Field `BGMODE` reader - Bandgap mode. Ready only."]
pub type BGMODE_R = crate::BitReader<BGMODE_A>;
#[doc = "Bandgap mode. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGMODE_A {
    #[doc = "0: Static mode (higher precision)"]
    BGMODE_0 = 0,
    #[doc = "1: Sampled mode (lower power consumption)"]
    BGMODE_1 = 1,
}
impl From<BGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BGMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGMODE_A {
        match self.bits {
            false => BGMODE_A::BGMODE_0,
            true => BGMODE_A::BGMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGMODE_0`"]
    #[inline(always)]
    pub fn is_bgmode_0(&self) -> bool {
        *self == BGMODE_A::BGMODE_0
    }
    #[doc = "Checks if the value of the field is `BGMODE_1`"]
    #[inline(always)]
    pub fn is_bgmode_1(&self) -> bool {
        *self == BGMODE_A::BGMODE_1
    }
}
#[doc = "Field `BGMODE` writer - Bandgap mode. Ready only."]
pub type BGMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, BGMODE_A, O>;
impl<'a, const O: u8> BGMODE_W<'a, O> {
    #[doc = "Static mode (higher precision)"]
    #[inline(always)]
    pub fn bgmode_0(self) -> &'a mut W {
        self.variant(BGMODE_A::BGMODE_0)
    }
    #[doc = "Sampled mode (lower power consumption)"]
    #[inline(always)]
    pub fn bgmode_1(self) -> &'a mut W {
        self.variant(BGMODE_A::BGMODE_1)
    }
}
#[doc = "Field `REFGENRDY` reader - Variable reference voltage ready status."]
pub type REFGENRDY_R = crate::BitReader<REFGENRDY_A>;
#[doc = "Variable reference voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENRDY_A {
    #[doc = "0: Reference voltage output is not ready to be used."]
    REFGENRDY_0 = 0,
    #[doc = "1: Reference voltage output is ready to be used"]
    REFGENRDY_1 = 1,
}
impl From<REFGENRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENRDY_A {
        match self.bits {
            false => REFGENRDY_A::REFGENRDY_0,
            true => REFGENRDY_A::REFGENRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_0`"]
    #[inline(always)]
    pub fn is_refgenrdy_0(&self) -> bool {
        *self == REFGENRDY_A::REFGENRDY_0
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_1`"]
    #[inline(always)]
    pub fn is_refgenrdy_1(&self) -> bool {
        *self == REFGENRDY_A::REFGENRDY_1
    }
}
#[doc = "Field `REFGENRDY` writer - Variable reference voltage ready status."]
pub type REFGENRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, REFGENRDY_A, O>;
impl<'a, const O: u8> REFGENRDY_W<'a, O> {
    #[doc = "Reference voltage output is not ready to be used."]
    #[inline(always)]
    pub fn refgenrdy_0(self) -> &'a mut W {
        self.variant(REFGENRDY_A::REFGENRDY_0)
    }
    #[doc = "Reference voltage output is ready to be used"]
    #[inline(always)]
    pub fn refgenrdy_1(self) -> &'a mut W {
        self.variant(REFGENRDY_A::REFGENRDY_1)
    }
}
#[doc = "Field `REFBGRDY` reader - Buffered bandgap voltage ready status."]
pub type REFBGRDY_R = crate::BitReader<REFBGRDY_A>;
#[doc = "Buffered bandgap voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGRDY_A {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    REFBGRDY_0 = 0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    REFBGRDY_1 = 1,
}
impl From<REFBGRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGRDY_A {
        match self.bits {
            false => REFBGRDY_A::REFBGRDY_0,
            true => REFBGRDY_A::REFBGRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_0`"]
    #[inline(always)]
    pub fn is_refbgrdy_0(&self) -> bool {
        *self == REFBGRDY_A::REFBGRDY_0
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_1`"]
    #[inline(always)]
    pub fn is_refbgrdy_1(&self) -> bool {
        *self == REFBGRDY_A::REFBGRDY_1
    }
}
#[doc = "Field `REFBGRDY` writer - Buffered bandgap voltage ready status."]
pub type REFBGRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, REFBGRDY_A, O>;
impl<'a, const O: u8> REFBGRDY_W<'a, O> {
    #[doc = "Buffered bandgap voltage is not ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_0(self) -> &'a mut W {
        self.variant(REFBGRDY_A::REFBGRDY_0)
    }
    #[doc = "Buffered bandgap voltage is ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_1(self) -> &'a mut W {
        self.variant(REFBGRDY_A::REFBGRDY_1)
    }
}
#[doc = "Field `PWRMODE` reader - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRMODE` writer - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PMMCTL2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> INTREFEN_R {
        INTREFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> EXTREFEN_R {
        EXTREFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TSENSOREN_R {
        TSENSOREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&self) -> REFGEN_R {
        REFGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&self) -> REFBGEN_R {
        REFBGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reference generator active. Read only."]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference bandgap active. Ready only."]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> INTREFEN_W<0> {
        INTREFEN_W::new(self)
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> EXTREFEN_W<1> {
        EXTREFEN_W::new(self)
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TSENSOREN_W<3> {
        TSENSOREN_W::new(self)
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W<4> {
        REFVSEL_W::new(self)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&mut self) -> REFGEN_W<6> {
        REFGEN_W::new(self)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> REFBGEN_W<7> {
        REFBGEN_W::new(self)
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W<11> {
        BGMODE_W::new(self)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W<12> {
        REFGENRDY_W::new(self)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W<13> {
        REFBGRDY_W::new(self)
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<14> {
        PWRMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management Module Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl2](index.html) module"]
pub struct PMMCTL2_SPEC;
impl crate::RegisterSpec for PMMCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl2::R](R) reader structure"]
impl crate::Readable for PMMCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl2::W](W) writer structure"]
impl crate::Writable for PMMCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for PMMCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
