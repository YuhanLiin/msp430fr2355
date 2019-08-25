#[doc = "Reader of register CSCTL6"]
pub type R = crate::R<u16, super::CSCTL6>;
#[doc = "Writer for register CSCTL6"]
pub type W = crate::W<u16, super::CSCTL6>;
#[doc = "Register CSCTL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL6 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1AUTOOFF_A {
    #[doc = "0: XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation."]
    XT1AUTOOFF_0,
    #[doc = "1: XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation."]
    XT1AUTOOFF_1,
}
impl From<XT1AUTOOFF_A> for bool {
    #[inline(always)]
    fn from(variant: XT1AUTOOFF_A) -> Self {
        match variant {
            XT1AUTOOFF_A::XT1AUTOOFF_0 => false,
            XT1AUTOOFF_A::XT1AUTOOFF_1 => true,
        }
    }
}
#[doc = "Reader of field `XT1AUTOOFF`"]
pub type XT1AUTOOFF_R = crate::R<bool, XT1AUTOOFF_A>;
impl XT1AUTOOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1AUTOOFF_A {
        match self.bits {
            false => XT1AUTOOFF_A::XT1AUTOOFF_0,
            true => XT1AUTOOFF_A::XT1AUTOOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `XT1AUTOOFF_0`"]
    #[inline(always)]
    pub fn is_xt1autooff_0(&self) -> bool {
        *self == XT1AUTOOFF_A::XT1AUTOOFF_0
    }
    #[doc = "Checks if the value of the field is `XT1AUTOOFF_1`"]
    #[inline(always)]
    pub fn is_xt1autooff_1(&self) -> bool {
        *self == XT1AUTOOFF_A::XT1AUTOOFF_1
    }
}
#[doc = "Write proxy for field `XT1AUTOOFF`"]
pub struct XT1AUTOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1AUTOOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1AUTOOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation."]
    #[inline(always)]
    pub fn xt1autooff_0(self) -> &'a mut W {
        self.variant(XT1AUTOOFF_A::XT1AUTOOFF_0)
    }
    #[doc = "XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation."]
    #[inline(always)]
    pub fn xt1autooff_1(self) -> &'a mut W {
        self.variant(XT1AUTOOFF_A::XT1AUTOOFF_1)
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
#[doc = "Automatic Gain Control (AGC) disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1AGCOFF_A {
    #[doc = "0: AGC on"]
    XT1AGCOFF_0,
    #[doc = "1: AGC off"]
    XT1AGCOFF_1,
}
impl From<XT1AGCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: XT1AGCOFF_A) -> Self {
        match variant {
            XT1AGCOFF_A::XT1AGCOFF_0 => false,
            XT1AGCOFF_A::XT1AGCOFF_1 => true,
        }
    }
}
#[doc = "Reader of field `XT1AGCOFF`"]
pub type XT1AGCOFF_R = crate::R<bool, XT1AGCOFF_A>;
impl XT1AGCOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1AGCOFF_A {
        match self.bits {
            false => XT1AGCOFF_A::XT1AGCOFF_0,
            true => XT1AGCOFF_A::XT1AGCOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `XT1AGCOFF_0`"]
    #[inline(always)]
    pub fn is_xt1agcoff_0(&self) -> bool {
        *self == XT1AGCOFF_A::XT1AGCOFF_0
    }
    #[doc = "Checks if the value of the field is `XT1AGCOFF_1`"]
    #[inline(always)]
    pub fn is_xt1agcoff_1(&self) -> bool {
        *self == XT1AGCOFF_A::XT1AGCOFF_1
    }
}
#[doc = "Write proxy for field `XT1AGCOFF`"]
pub struct XT1AGCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1AGCOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1AGCOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AGC on"]
    #[inline(always)]
    pub fn xt1agcoff_0(self) -> &'a mut W {
        self.variant(XT1AGCOFF_A::XT1AGCOFF_0)
    }
    #[doc = "AGC off"]
    #[inline(always)]
    pub fn xt1agcoff_1(self) -> &'a mut W {
        self.variant(XT1AGCOFF_A::XT1AGCOFF_1)
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
#[doc = "The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1HFFREQ_A {
    #[doc = "0: 1 to 4 MHz"]
    XT1HFFREQ_0,
    #[doc = "1: 4 MHz to 6 MHz"]
    XT1HFFREQ_1,
    #[doc = "2: 6 MHz to 16 MHz"]
    XT1HFFREQ_2,
    #[doc = "3: 16 MHz to 24 MHz"]
    XT1HFFREQ_3,
}
impl From<XT1HFFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1HFFREQ_A) -> Self {
        match variant {
            XT1HFFREQ_A::XT1HFFREQ_0 => 0,
            XT1HFFREQ_A::XT1HFFREQ_1 => 1,
            XT1HFFREQ_A::XT1HFFREQ_2 => 2,
            XT1HFFREQ_A::XT1HFFREQ_3 => 3,
        }
    }
}
#[doc = "Reader of field `XT1HFFREQ`"]
pub type XT1HFFREQ_R = crate::R<u8, XT1HFFREQ_A>;
impl XT1HFFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1HFFREQ_A {
        match self.bits {
            0 => XT1HFFREQ_A::XT1HFFREQ_0,
            1 => XT1HFFREQ_A::XT1HFFREQ_1,
            2 => XT1HFFREQ_A::XT1HFFREQ_2,
            3 => XT1HFFREQ_A::XT1HFFREQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1HFFREQ_0`"]
    #[inline(always)]
    pub fn is_xt1hffreq_0(&self) -> bool {
        *self == XT1HFFREQ_A::XT1HFFREQ_0
    }
    #[doc = "Checks if the value of the field is `XT1HFFREQ_1`"]
    #[inline(always)]
    pub fn is_xt1hffreq_1(&self) -> bool {
        *self == XT1HFFREQ_A::XT1HFFREQ_1
    }
    #[doc = "Checks if the value of the field is `XT1HFFREQ_2`"]
    #[inline(always)]
    pub fn is_xt1hffreq_2(&self) -> bool {
        *self == XT1HFFREQ_A::XT1HFFREQ_2
    }
    #[doc = "Checks if the value of the field is `XT1HFFREQ_3`"]
    #[inline(always)]
    pub fn is_xt1hffreq_3(&self) -> bool {
        *self == XT1HFFREQ_A::XT1HFFREQ_3
    }
}
#[doc = "Write proxy for field `XT1HFFREQ`"]
pub struct XT1HFFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1HFFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1HFFREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 to 4 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_0(self) -> &'a mut W {
        self.variant(XT1HFFREQ_A::XT1HFFREQ_0)
    }
    #[doc = "4 MHz to 6 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_1(self) -> &'a mut W {
        self.variant(XT1HFFREQ_A::XT1HFFREQ_1)
    }
    #[doc = "6 MHz to 16 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_2(self) -> &'a mut W {
        self.variant(XT1HFFREQ_A::XT1HFFREQ_2)
    }
    #[doc = "16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_3(self) -> &'a mut W {
        self.variant(XT1HFFREQ_A::XT1HFFREQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "XT1 bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1BYPASS_A {
    #[doc = "0: XT1 source internally"]
    XT1BYPASS_0,
    #[doc = "1: XT1 sources externally from pin"]
    XT1BYPASS_1,
}
impl From<XT1BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: XT1BYPASS_A) -> Self {
        match variant {
            XT1BYPASS_A::XT1BYPASS_0 => false,
            XT1BYPASS_A::XT1BYPASS_1 => true,
        }
    }
}
#[doc = "Reader of field `XT1BYPASS`"]
pub type XT1BYPASS_R = crate::R<bool, XT1BYPASS_A>;
impl XT1BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1BYPASS_A {
        match self.bits {
            false => XT1BYPASS_A::XT1BYPASS_0,
            true => XT1BYPASS_A::XT1BYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `XT1BYPASS_0`"]
    #[inline(always)]
    pub fn is_xt1bypass_0(&self) -> bool {
        *self == XT1BYPASS_A::XT1BYPASS_0
    }
    #[doc = "Checks if the value of the field is `XT1BYPASS_1`"]
    #[inline(always)]
    pub fn is_xt1bypass_1(&self) -> bool {
        *self == XT1BYPASS_A::XT1BYPASS_1
    }
}
#[doc = "Write proxy for field `XT1BYPASS`"]
pub struct XT1BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XT1 source internally"]
    #[inline(always)]
    pub fn xt1bypass_0(self) -> &'a mut W {
        self.variant(XT1BYPASS_A::XT1BYPASS_0)
    }
    #[doc = "XT1 sources externally from pin"]
    #[inline(always)]
    pub fn xt1bypass_1(self) -> &'a mut W {
        self.variant(XT1BYPASS_A::XT1BYPASS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "XT1 mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTS_A {
    #[doc = "0: Low-frequency mode."]
    XTS_0,
    #[doc = "1: High-frequency mode."]
    XTS_1,
}
impl From<XTS_A> for bool {
    #[inline(always)]
    fn from(variant: XTS_A) -> Self {
        match variant {
            XTS_A::XTS_0 => false,
            XTS_A::XTS_1 => true,
        }
    }
}
#[doc = "Reader of field `XTS`"]
pub type XTS_R = crate::R<bool, XTS_A>;
impl XTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTS_A {
        match self.bits {
            false => XTS_A::XTS_0,
            true => XTS_A::XTS_1,
        }
    }
    #[doc = "Checks if the value of the field is `XTS_0`"]
    #[inline(always)]
    pub fn is_xts_0(&self) -> bool {
        *self == XTS_A::XTS_0
    }
    #[doc = "Checks if the value of the field is `XTS_1`"]
    #[inline(always)]
    pub fn is_xts_1(&self) -> bool {
        *self == XTS_A::XTS_1
    }
}
#[doc = "Write proxy for field `XTS`"]
pub struct XTS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low-frequency mode."]
    #[inline(always)]
    pub fn xts_0(self) -> &'a mut W {
        self.variant(XTS_A::XTS_0)
    }
    #[doc = "High-frequency mode."]
    #[inline(always)]
    pub fn xts_1(self) -> &'a mut W {
        self.variant(XTS_A::XTS_1)
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
#[doc = "The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1DRIVE_A {
    #[doc = "0: Lowest drive strength and current consumption"]
    XT1DRIVE_0,
    #[doc = "1: Lower drive strength and current consumption"]
    XT1DRIVE_1,
    #[doc = "2: Higher drive strength and current consumption"]
    XT1DRIVE_2,
    #[doc = "3: Highest drive strength and current consumption"]
    XT1DRIVE_3,
}
impl From<XT1DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1DRIVE_A) -> Self {
        match variant {
            XT1DRIVE_A::XT1DRIVE_0 => 0,
            XT1DRIVE_A::XT1DRIVE_1 => 1,
            XT1DRIVE_A::XT1DRIVE_2 => 2,
            XT1DRIVE_A::XT1DRIVE_3 => 3,
        }
    }
}
#[doc = "Reader of field `XT1DRIVE`"]
pub type XT1DRIVE_R = crate::R<u8, XT1DRIVE_A>;
impl XT1DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1DRIVE_A {
        match self.bits {
            0 => XT1DRIVE_A::XT1DRIVE_0,
            1 => XT1DRIVE_A::XT1DRIVE_1,
            2 => XT1DRIVE_A::XT1DRIVE_2,
            3 => XT1DRIVE_A::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_0`"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_1`"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_2`"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_3`"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_3
    }
}
#[doc = "Write proxy for field `XT1DRIVE`"]
pub struct XT1DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lowest drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_0)
    }
    #[doc = "Lower drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_1)
    }
    #[doc = "Higher drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_2)
    }
    #[doc = "Highest drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "ACLK source divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVA_A {
    #[doc = "0: /1"]
    _1,
    #[doc = "1: /16"]
    _16,
    #[doc = "2: /32"]
    _32,
    #[doc = "3: /64"]
    _64,
    #[doc = "4: /128"]
    _128,
    #[doc = "5: /256"]
    _256,
    #[doc = "6: /384"]
    _384,
    #[doc = "7: /512"]
    _512,
    #[doc = "8: /768(Only available in 24MHz clock system, 24 MHz preference)"]
    _768,
    #[doc = "9: /1024(Only available in 24MHz clock system, 24 MHz preference)"]
    _1024,
    #[doc = "10: /108(Only available in 24MHz clock system, 24 MHz preference)"]
    _108,
    #[doc = "11: 338(Only available in 24MHz clock system, 24 MHz preference)"]
    _338,
    #[doc = "12: 414(Only available in 24MHz clock system, 24 MHz preference)"]
    _414,
    #[doc = "13: 640(Only available in 24MHz clock system, 24 MHz preference)"]
    _640,
    #[doc = "14: Reserved"]
    DIVA_14,
    #[doc = "15: Reserved"]
    DIVA_15,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        match variant {
            DIVA_A::_1 => 0,
            DIVA_A::_16 => 1,
            DIVA_A::_32 => 2,
            DIVA_A::_64 => 3,
            DIVA_A::_128 => 4,
            DIVA_A::_256 => 5,
            DIVA_A::_384 => 6,
            DIVA_A::_512 => 7,
            DIVA_A::_768 => 8,
            DIVA_A::_1024 => 9,
            DIVA_A::_108 => 10,
            DIVA_A::_338 => 11,
            DIVA_A::_414 => 12,
            DIVA_A::_640 => 13,
            DIVA_A::DIVA_14 => 14,
            DIVA_A::DIVA_15 => 15,
        }
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::_1,
            1 => DIVA_A::_16,
            2 => DIVA_A::_32,
            3 => DIVA_A::_64,
            4 => DIVA_A::_128,
            5 => DIVA_A::_256,
            6 => DIVA_A::_384,
            7 => DIVA_A::_512,
            8 => DIVA_A::_768,
            9 => DIVA_A::_1024,
            10 => DIVA_A::_108,
            11 => DIVA_A::_338,
            12 => DIVA_A::_414,
            13 => DIVA_A::_640,
            14 => DIVA_A::DIVA_14,
            15 => DIVA_A::DIVA_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVA_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DIVA_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DIVA_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == DIVA_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DIVA_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == DIVA_A::_256
    }
    #[doc = "Checks if the value of the field is `_384`"]
    #[inline(always)]
    pub fn is_384(&self) -> bool {
        *self == DIVA_A::_384
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == DIVA_A::_512
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == DIVA_A::_768
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == DIVA_A::_1024
    }
    #[doc = "Checks if the value of the field is `_108`"]
    #[inline(always)]
    pub fn is_108(&self) -> bool {
        *self == DIVA_A::_108
    }
    #[doc = "Checks if the value of the field is `_338`"]
    #[inline(always)]
    pub fn is_338(&self) -> bool {
        *self == DIVA_A::_338
    }
    #[doc = "Checks if the value of the field is `_414`"]
    #[inline(always)]
    pub fn is_414(&self) -> bool {
        *self == DIVA_A::_414
    }
    #[doc = "Checks if the value of the field is `_640`"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == DIVA_A::_640
    }
    #[doc = "Checks if the value of the field is `DIVA_14`"]
    #[inline(always)]
    pub fn is_diva_14(&self) -> bool {
        *self == DIVA_A::DIVA_14
    }
    #[doc = "Checks if the value of the field is `DIVA_15`"]
    #[inline(always)]
    pub fn is_diva_15(&self) -> bool {
        *self == DIVA_A::DIVA_15
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVA_A::_1)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DIVA_A::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DIVA_A::_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(DIVA_A::_64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DIVA_A::_128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(DIVA_A::_256)
    }
    #[doc = "/384"]
    #[inline(always)]
    pub fn _384(self) -> &'a mut W {
        self.variant(DIVA_A::_384)
    }
    #[doc = "/512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(DIVA_A::_512)
    }
    #[doc = "/768(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(DIVA_A::_768)
    }
    #[doc = "/1024(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DIVA_A::_1024)
    }
    #[doc = "/108(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _108(self) -> &'a mut W {
        self.variant(DIVA_A::_108)
    }
    #[doc = "338(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _338(self) -> &'a mut W {
        self.variant(DIVA_A::_338)
    }
    #[doc = "414(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _414(self) -> &'a mut W {
        self.variant(DIVA_A::_414)
    }
    #[doc = "640(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut W {
        self.variant(DIVA_A::_640)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn diva_14(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn diva_15(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "The XT1 oscillator fault detection off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1FAULTOFF_A {
    #[doc = "0: Enabling XT1 fault to switch ACLK to REFO"]
    XT1FAULTOFF_0,
    #[doc = "1: Disabling XT1 fault to switch ACLK to REFO"]
    XT1FAULTOFF_1,
}
impl From<XT1FAULTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: XT1FAULTOFF_A) -> Self {
        match variant {
            XT1FAULTOFF_A::XT1FAULTOFF_0 => false,
            XT1FAULTOFF_A::XT1FAULTOFF_1 => true,
        }
    }
}
#[doc = "Reader of field `XT1FAULTOFF`"]
pub type XT1FAULTOFF_R = crate::R<bool, XT1FAULTOFF_A>;
impl XT1FAULTOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1FAULTOFF_A {
        match self.bits {
            false => XT1FAULTOFF_A::XT1FAULTOFF_0,
            true => XT1FAULTOFF_A::XT1FAULTOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `XT1FAULTOFF_0`"]
    #[inline(always)]
    pub fn is_xt1faultoff_0(&self) -> bool {
        *self == XT1FAULTOFF_A::XT1FAULTOFF_0
    }
    #[doc = "Checks if the value of the field is `XT1FAULTOFF_1`"]
    #[inline(always)]
    pub fn is_xt1faultoff_1(&self) -> bool {
        *self == XT1FAULTOFF_A::XT1FAULTOFF_1
    }
}
#[doc = "Write proxy for field `XT1FAULTOFF`"]
pub struct XT1FAULTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1FAULTOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1FAULTOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn xt1faultoff_0(self) -> &'a mut W {
        self.variant(XT1FAULTOFF_A::XT1FAULTOFF_0)
    }
    #[doc = "Disabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn xt1faultoff_1(self) -> &'a mut W {
        self.variant(XT1FAULTOFF_A::XT1FAULTOFF_1)
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
impl R {
    #[doc = "Bit 0 - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
    #[inline(always)]
    pub fn xt1autooff(&self) -> XT1AUTOOFF_R {
        XT1AUTOOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Automatic Gain Control (AGC) disable."]
    #[inline(always)]
    pub fn xt1agcoff(&self) -> XT1AGCOFF_R {
        XT1AGCOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
    #[inline(always)]
    pub fn xt1hffreq(&self) -> XT1HFFREQ_R {
        XT1HFFREQ_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - XT1 bypass select"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> XT1BYPASS_R {
        XT1BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XT1 mode select"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
    #[inline(always)]
    pub fn xt1drive(&self) -> XT1DRIVE_R {
        XT1DRIVE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - ACLK source divider."]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - The XT1 oscillator fault detection off"]
    #[inline(always)]
    pub fn xt1faultoff(&self) -> XT1FAULTOFF_R {
        XT1FAULTOFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
    #[inline(always)]
    pub fn xt1autooff(&mut self) -> XT1AUTOOFF_W {
        XT1AUTOOFF_W { w: self }
    }
    #[doc = "Bit 1 - Automatic Gain Control (AGC) disable."]
    #[inline(always)]
    pub fn xt1agcoff(&mut self) -> XT1AGCOFF_W {
        XT1AGCOFF_W { w: self }
    }
    #[doc = "Bits 2:3 - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
    #[inline(always)]
    pub fn xt1hffreq(&mut self) -> XT1HFFREQ_W {
        XT1HFFREQ_W { w: self }
    }
    #[doc = "Bit 4 - XT1 bypass select"]
    #[inline(always)]
    pub fn xt1bypass(&mut self) -> XT1BYPASS_W {
        XT1BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - XT1 mode select"]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W {
        XTS_W { w: self }
    }
    #[doc = "Bits 6:7 - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> XT1DRIVE_W {
        XT1DRIVE_W { w: self }
    }
    #[doc = "Bits 8:11 - ACLK source divider."]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bit 13 - The XT1 oscillator fault detection off"]
    #[inline(always)]
    pub fn xt1faultoff(&mut self) -> XT1FAULTOFF_W {
        XT1FAULTOFF_W { w: self }
    }
}
