#[doc = "Reader of register PMMCTL2"]
pub type R = crate::R<u16, super::PMMCTL2>;
#[doc = "Writer for register PMMCTL2"]
pub type W = crate::W<u16, super::PMMCTL2>;
#[doc = "Register PMMCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Internal reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTREFEN_A {
    #[doc = "0: Disable internal reference"]
    INTREFEN_0,
    #[doc = "1: Enable internal reference"]
    INTREFEN_1,
}
impl From<INTREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTREFEN_A) -> Self {
        match variant {
            INTREFEN_A::INTREFEN_0 => false,
            INTREFEN_A::INTREFEN_1 => true,
        }
    }
}
#[doc = "Reader of field `INTREFEN`"]
pub type INTREFEN_R = crate::R<bool, INTREFEN_A>;
impl INTREFEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `INTREFEN`"]
pub struct INTREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "External reference output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFEN_A {
    #[doc = "0: Disable external reference output"]
    EXTREFEN_0,
    #[doc = "1: Enable internal reference output"]
    EXTREFEN_1,
}
impl From<EXTREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTREFEN_A) -> Self {
        match variant {
            EXTREFEN_A::EXTREFEN_0 => false,
            EXTREFEN_A::EXTREFEN_1 => true,
        }
    }
}
#[doc = "Reader of field `EXTREFEN`"]
pub type EXTREFEN_R = crate::R<bool, EXTREFEN_A>;
impl EXTREFEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `EXTREFEN`"]
pub struct EXTREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRMODE_A {}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `PWRMODE`"]
pub type PWRMODE_R = crate::R<u8, PWRMODE_A>;
impl PWRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRMODE_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `PWRMODE`"]
pub struct PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSENSOREN_A {
    #[doc = "0: Disable temperature sensor"]
    TSENSOREN_0,
    #[doc = "1: Enable temperature sensor"]
    TSENSOREN_1,
}
impl From<TSENSOREN_A> for bool {
    #[inline(always)]
    fn from(variant: TSENSOREN_A) -> Self {
        match variant {
            TSENSOREN_A::TSENSOREN_0 => false,
            TSENSOREN_A::TSENSOREN_1 => true,
        }
    }
}
#[doc = "Reader of field `TSENSOREN`"]
pub type TSENSOREN_R = crate::R<bool, TSENSOREN_A>;
impl TSENSOREN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `TSENSOREN`"]
pub struct TSENSOREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENSOREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSENSOREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reference generator active. Read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENACT_A {
    #[doc = "0: Reference generator not active"]
    REFGENACT_0,
    #[doc = "1: Reference generator active"]
    REFGENACT_1,
}
impl From<REFGENACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENACT_A) -> Self {
        match variant {
            REFGENACT_A::REFGENACT_0 => false,
            REFGENACT_A::REFGENACT_1 => true,
        }
    }
}
#[doc = "Reader of field `REFGENACT`"]
pub type REFGENACT_R = crate::R<bool, REFGENACT_A>;
impl REFGENACT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Reference bandgap active. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGACT_A {
    #[doc = "0: Reference bandgap buffer not active"]
    REFBGACT_0,
    #[doc = "1: Reference bandgap buffer active"]
    REFBGACT_1,
}
impl From<REFBGACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGACT_A) -> Self {
        match variant {
            REFBGACT_A::REFBGACT_0 => false,
            REFBGACT_A::REFBGACT_1 => true,
        }
    }
}
#[doc = "Reader of field `REFBGACT`"]
pub type REFBGACT_R = crate::R<bool, REFBGACT_A>;
impl REFBGACT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Bandgap mode. Ready only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGMODE_A {
    #[doc = "0: Static mode (higher precision)"]
    BGMODE_0,
    #[doc = "1: Sampled mode (lower power consumption)"]
    BGMODE_1,
}
impl From<BGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BGMODE_A) -> Self {
        match variant {
            BGMODE_A::BGMODE_0 => false,
            BGMODE_A::BGMODE_1 => true,
        }
    }
}
#[doc = "Reader of field `BGMODE`"]
pub type BGMODE_R = crate::R<bool, BGMODE_A>;
impl BGMODE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `BGMODE`"]
pub struct BGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Variable reference voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENRDY_A {
    #[doc = "0: Reference voltage output is not ready to be used."]
    REFGENRDY_0,
    #[doc = "1: Reference voltage output is ready to be used"]
    REFGENRDY_1,
}
impl From<REFGENRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENRDY_A) -> Self {
        match variant {
            REFGENRDY_A::REFGENRDY_0 => false,
            REFGENRDY_A::REFGENRDY_1 => true,
        }
    }
}
#[doc = "Reader of field `REFGENRDY`"]
pub type REFGENRDY_R = crate::R<bool, REFGENRDY_A>;
impl REFGENRDY_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `REFGENRDY`"]
pub struct REFGENRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGENRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Buffered bandgap voltage ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGRDY_A {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    REFBGRDY_0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    REFBGRDY_1,
}
impl From<REFBGRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGRDY_A) -> Self {
        match variant {
            REFBGRDY_A::REFBGRDY_0 => false,
            REFBGRDY_A::REFBGRDY_1 => true,
        }
    }
}
#[doc = "Reader of field `REFBGRDY`"]
pub type REFBGRDY_R = crate::R<bool, REFBGRDY_A>;
impl REFBGRDY_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `REFBGRDY`"]
pub struct REFBGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBGRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reference voltage level select. Can be modified only when REFGENBUSY = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFVSEL_A {
    #[doc = "0: 00b = 1.5V"]
    REFVSEL_0,
    #[doc = "1: 01b = 2.0V"]
    REFVSEL_1,
    #[doc = "2: 10b = 2.5V"]
    REFVSEL_2,
    #[doc = "3: 11b = Reserved"]
    REFVSEL_3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        match variant {
            REFVSEL_A::REFVSEL_0 => 0,
            REFVSEL_A::REFVSEL_1 => 1,
            REFVSEL_A::REFVSEL_2 => 2,
            REFVSEL_A::REFVSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `REFVSEL`"]
pub type REFVSEL_R = crate::R<u8, REFVSEL_A>;
impl REFVSEL_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `REFVSEL`"]
pub struct REFVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGEN_A {
    #[doc = "0: No trigger"]
    REFGEN_0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    REFGEN_1,
}
impl From<REFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REFGEN_A) -> Self {
        match variant {
            REFGEN_A::REFGEN_0 => false,
            REFGEN_A::REFGEN_1 => true,
        }
    }
}
#[doc = "Reader of field `REFGEN`"]
pub type REFGEN_R = crate::R<bool, REFGEN_A>;
impl REFGEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `REFGEN`"]
pub struct REFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGEN_A {
    #[doc = "0: No trigger"]
    REFBG_0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    REFBG_1,
}
impl From<REFBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGEN_A) -> Self {
        match variant {
            REFBGEN_A::REFBG_0 => false,
            REFBGEN_A::REFBG_1 => true,
        }
    }
}
#[doc = "Reader of field `REFBGEN`"]
pub type REFBGEN_R = crate::R<bool, REFBGEN_A>;
impl REFBGEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `REFBGEN`"]
pub struct REFBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> INTREFEN_R {
        INTREFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> EXTREFEN_R {
        EXTREFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TSENSOREN_R {
        TSENSOREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reference generator active. Read only."]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reference bandgap active. Ready only."]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&self) -> REFGEN_R {
        REFGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&self) -> REFBGEN_R {
        REFBGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal reference enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> INTREFEN_W {
        INTREFEN_W { w: self }
    }
    #[doc = "Bit 1 - External reference output enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> EXTREFEN_W {
        EXTREFEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Power Mode Selection. The two bits are used to select the power supply in multi power supply systems. A single power supply system is not affected by the bits. Reserved for future use."]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W {
        PWRMODE_W { w: self }
    }
    #[doc = "Bit 3 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TSENSOREN_W {
        TSENSOREN_W { w: self }
    }
    #[doc = "Bit 11 - Bandgap mode. Ready only."]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W {
        BGMODE_W { w: self }
    }
    #[doc = "Bit 12 - Variable reference voltage ready status."]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W {
        REFGENRDY_W { w: self }
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status."]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W {
        REFBGRDY_W { w: self }
    }
    #[doc = "Bits 4:5 - Reference voltage level select. Can be modified only when REFGENBUSY = 0."]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W {
        REFVSEL_W { w: self }
    }
    #[doc = "Bit 6 - Reference generator one-time trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refgen(&mut self) -> REFGEN_W {
        REFGEN_W { w: self }
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> REFBGEN_W {
        REFBGEN_W { w: self }
    }
}
