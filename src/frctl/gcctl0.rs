#[doc = "Reader of register GCCTL0"]
pub type R = crate::R<u16, super::GCCTL0>;
#[doc = "Writer for register GCCTL0"]
pub type W = crate::W<u16, super::GCCTL0>;
#[doc = "Register GCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UBDRSTEN_A {
    #[doc = "0: PUC not initiated on uncorrectable bit error detection flag."]
    UBDRSTEN_0 = 0,
    #[doc = "1: PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    UBDRSTEN_1 = 1,
}
impl From<UBDRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UBDRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UBDRSTEN`"]
pub type UBDRSTEN_R = crate::R<bool, UBDRSTEN_A>;
impl UBDRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDRSTEN_A {
        match self.bits {
            false => UBDRSTEN_A::UBDRSTEN_0,
            true => UBDRSTEN_A::UBDRSTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDRSTEN_0`"]
    #[inline(always)]
    pub fn is_ubdrsten_0(&self) -> bool {
        *self == UBDRSTEN_A::UBDRSTEN_0
    }
    #[doc = "Checks if the value of the field is `UBDRSTEN_1`"]
    #[inline(always)]
    pub fn is_ubdrsten_1(&self) -> bool {
        *self == UBDRSTEN_A::UBDRSTEN_1
    }
}
#[doc = "Write proxy for field `UBDRSTEN`"]
pub struct UBDRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UBDRSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PUC not initiated on uncorrectable bit error detection flag."]
    #[inline(always)]
    pub fn ubdrsten_0(self) -> &'a mut W {
        self.variant(UBDRSTEN_A::UBDRSTEN_0)
    }
    #[doc = "PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    #[inline(always)]
    pub fn ubdrsten_1(self) -> &'a mut W {
        self.variant(UBDRSTEN_A::UBDRSTEN_1)
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
#[doc = "Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UBDIE_A {
    #[doc = "0: Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    UBDIE_0 = 0,
    #[doc = "1: Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    UBDIE_1 = 1,
}
impl From<UBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: UBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UBDIE`"]
pub type UBDIE_R = crate::R<bool, UBDIE_A>;
impl UBDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDIE_A {
        match self.bits {
            false => UBDIE_A::UBDIE_0,
            true => UBDIE_A::UBDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDIE_0`"]
    #[inline(always)]
    pub fn is_ubdie_0(&self) -> bool {
        *self == UBDIE_A::UBDIE_0
    }
    #[doc = "Checks if the value of the field is `UBDIE_1`"]
    #[inline(always)]
    pub fn is_ubdie_1(&self) -> bool {
        *self == UBDIE_A::UBDIE_1
    }
}
#[doc = "Write proxy for field `UBDIE`"]
pub struct UBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UBDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    #[inline(always)]
    pub fn ubdie_0(self) -> &'a mut W {
        self.variant(UBDIE_A::UBDIE_0)
    }
    #[doc = "Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    #[inline(always)]
    pub fn ubdie_1(self) -> &'a mut W {
        self.variant(UBDIE_A::UBDIE_1)
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
#[doc = "Enable NMI event for the correctable bit error detection flag (CBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBDIE_A {
    #[doc = "0: Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    CBDIE_0 = 0,
    #[doc = "1: Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    CBDIE_1 = 1,
}
impl From<CBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CBDIE`"]
pub type CBDIE_R = crate::R<bool, CBDIE_A>;
impl CBDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBDIE_A {
        match self.bits {
            false => CBDIE_A::CBDIE_0,
            true => CBDIE_A::CBDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CBDIE_0`"]
    #[inline(always)]
    pub fn is_cbdie_0(&self) -> bool {
        *self == CBDIE_A::CBDIE_0
    }
    #[doc = "Checks if the value of the field is `CBDIE_1`"]
    #[inline(always)]
    pub fn is_cbdie_1(&self) -> bool {
        *self == CBDIE_A::CBDIE_1
    }
}
#[doc = "Write proxy for field `CBDIE`"]
pub struct CBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    #[inline(always)]
    pub fn cbdie_0(self) -> &'a mut W {
        self.variant(CBDIE_A::CBDIE_0)
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    #[inline(always)]
    pub fn cbdie_1(self) -> &'a mut W {
        self.variant(CBDIE_A::CBDIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "FRAM Memory Power Control Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRPWR_A {
    #[doc = "0: Enable INACTIVE mode."]
    FRPWR_0 = 0,
    #[doc = "1: Enable ACTIVE mode."]
    FRPWR_1 = 1,
}
impl From<FRPWR_A> for bool {
    #[inline(always)]
    fn from(variant: FRPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRPWR`"]
pub type FRPWR_R = crate::R<bool, FRPWR_A>;
impl FRPWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRPWR_A {
        match self.bits {
            false => FRPWR_A::FRPWR_0,
            true => FRPWR_A::FRPWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRPWR_0`"]
    #[inline(always)]
    pub fn is_frpwr_0(&self) -> bool {
        *self == FRPWR_A::FRPWR_0
    }
    #[doc = "Checks if the value of the field is `FRPWR_1`"]
    #[inline(always)]
    pub fn is_frpwr_1(&self) -> bool {
        *self == FRPWR_A::FRPWR_1
    }
}
#[doc = "Write proxy for field `FRPWR`"]
pub struct FRPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRPWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable INACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_0(self) -> &'a mut W {
        self.variant(FRPWR_A::FRPWR_0)
    }
    #[doc = "Enable ACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_1(self) -> &'a mut W {
        self.variant(FRPWR_A::FRPWR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enables FRAM auto power up after LPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRLPMPWR_A {
    #[doc = "0: FRAM startup is delayed to the first FRAM access after exit from LPM"]
    FRLPMPWR_0 = 0,
    #[doc = "1: FRAM is powered up immediately on exit from LPM"]
    FRLPMPWR_1 = 1,
}
impl From<FRLPMPWR_A> for bool {
    #[inline(always)]
    fn from(variant: FRLPMPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRLPMPWR`"]
pub type FRLPMPWR_R = crate::R<bool, FRLPMPWR_A>;
impl FRLPMPWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRLPMPWR_A {
        match self.bits {
            false => FRLPMPWR_A::FRLPMPWR_0,
            true => FRLPMPWR_A::FRLPMPWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRLPMPWR_0`"]
    #[inline(always)]
    pub fn is_frlpmpwr_0(&self) -> bool {
        *self == FRLPMPWR_A::FRLPMPWR_0
    }
    #[doc = "Checks if the value of the field is `FRLPMPWR_1`"]
    #[inline(always)]
    pub fn is_frlpmpwr_1(&self) -> bool {
        *self == FRLPMPWR_A::FRLPMPWR_1
    }
}
#[doc = "Write proxy for field `FRLPMPWR`"]
pub struct FRLPMPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLPMPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRLPMPWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRAM startup is delayed to the first FRAM access after exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_0(self) -> &'a mut W {
        self.variant(FRLPMPWR_A::FRLPMPWR_0)
    }
    #[doc = "FRAM is powered up immediately on exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_1(self) -> &'a mut W {
        self.variant(FRLPMPWR_A::FRLPMPWR_1)
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
impl R {
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UBDRSTEN_R {
        UBDRSTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&self) -> UBDIE_R {
        UBDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&self) -> CBDIE_R {
        CBDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&self) -> FRPWR_R {
        FRPWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FRLPMPWR_R {
        FRLPMPWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UBDRSTEN_W {
        UBDRSTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UBDIE_W {
        UBDIE_W { w: self }
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CBDIE_W {
        CBDIE_W { w: self }
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FRPWR_W {
        FRPWR_W { w: self }
    }
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FRLPMPWR_W {
        FRLPMPWR_W { w: self }
    }
}
