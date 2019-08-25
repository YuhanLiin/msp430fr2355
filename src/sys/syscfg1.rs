#[doc = "Reader of register SYSCFG1"]
pub type R = crate::R<u16, super::SYSCFG1>;
#[doc = "Writer for register SYSCFG1"]
pub type W = crate::W<u16, super::SYSCFG1>;
#[doc = "Register SYSCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: Infrared function disabled"]
    IREN_0,
    #[doc = "1: Infrared function enabled"]
    IREN_1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        match variant {
            IREN_A::IREN_0 => false,
            IREN_A::IREN_1 => true,
        }
    }
}
#[doc = "Reader of field `IREN`"]
pub type IREN_R = crate::R<bool, IREN_A>;
impl IREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::IREN_0,
            true => IREN_A::IREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREN_0`"]
    #[inline(always)]
    pub fn is_iren_0(&self) -> bool {
        *self == IREN_A::IREN_0
    }
    #[doc = "Checks if the value of the field is `IREN_1`"]
    #[inline(always)]
    pub fn is_iren_1(&self) -> bool {
        *self == IREN_A::IREN_1
    }
}
#[doc = "Write proxy for field `IREN`"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Infrared function disabled"]
    #[inline(always)]
    pub fn iren_0(self) -> &'a mut W {
        self.variant(IREN_A::IREN_0)
    }
    #[doc = "Infrared function enabled"]
    #[inline(always)]
    pub fn iren_1(self) -> &'a mut W {
        self.variant(IREN_A::IREN_1)
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
#[doc = "Infrared polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRPSEL_A {
    #[doc = "0: Normal polarity"]
    IRPSEL_0,
    #[doc = "1: Inverted polarity"]
    IRPSEL_1,
}
impl From<IRPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRPSEL_A) -> Self {
        match variant {
            IRPSEL_A::IRPSEL_0 => false,
            IRPSEL_A::IRPSEL_1 => true,
        }
    }
}
#[doc = "Reader of field `IRPSEL`"]
pub type IRPSEL_R = crate::R<bool, IRPSEL_A>;
impl IRPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPSEL_A {
        match self.bits {
            false => IRPSEL_A::IRPSEL_0,
            true => IRPSEL_A::IRPSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRPSEL_0`"]
    #[inline(always)]
    pub fn is_irpsel_0(&self) -> bool {
        *self == IRPSEL_A::IRPSEL_0
    }
    #[doc = "Checks if the value of the field is `IRPSEL_1`"]
    #[inline(always)]
    pub fn is_irpsel_1(&self) -> bool {
        *self == IRPSEL_A::IRPSEL_1
    }
}
#[doc = "Write proxy for field `IRPSEL`"]
pub struct IRPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal polarity"]
    #[inline(always)]
    pub fn irpsel_0(self) -> &'a mut W {
        self.variant(IRPSEL_A::IRPSEL_0)
    }
    #[doc = "Inverted polarity"]
    #[inline(always)]
    pub fn irpsel_1(self) -> &'a mut W {
        self.variant(IRPSEL_A::IRPSEL_1)
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
#[doc = "Infrared mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMSEL_A {
    #[doc = "0: FSK mode"]
    IRMSEL_0,
    #[doc = "1: ASK mode"]
    IRMSEL_1,
}
impl From<IRMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRMSEL_A) -> Self {
        match variant {
            IRMSEL_A::IRMSEL_0 => false,
            IRMSEL_A::IRMSEL_1 => true,
        }
    }
}
#[doc = "Reader of field `IRMSEL`"]
pub type IRMSEL_R = crate::R<bool, IRMSEL_A>;
impl IRMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMSEL_A {
        match self.bits {
            false => IRMSEL_A::IRMSEL_0,
            true => IRMSEL_A::IRMSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRMSEL_0`"]
    #[inline(always)]
    pub fn is_irmsel_0(&self) -> bool {
        *self == IRMSEL_A::IRMSEL_0
    }
    #[doc = "Checks if the value of the field is `IRMSEL_1`"]
    #[inline(always)]
    pub fn is_irmsel_1(&self) -> bool {
        *self == IRMSEL_A::IRMSEL_1
    }
}
#[doc = "Write proxy for field `IRMSEL`"]
pub struct IRMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FSK mode"]
    #[inline(always)]
    pub fn irmsel_0(self) -> &'a mut W {
        self.variant(IRMSEL_A::IRMSEL_0)
    }
    #[doc = "ASK mode"]
    #[inline(always)]
    pub fn irmsel_1(self) -> &'a mut W {
        self.variant(IRMSEL_A::IRMSEL_1)
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
#[doc = "Infrared data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDSSEL_A {
    #[doc = "0: From hardware peripherals upon device configuration"]
    IRDSSEL_0,
    #[doc = "1: From IRDATA bit"]
    IRDSSEL_1,
}
impl From<IRDSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRDSSEL_A) -> Self {
        match variant {
            IRDSSEL_A::IRDSSEL_0 => false,
            IRDSSEL_A::IRDSSEL_1 => true,
        }
    }
}
#[doc = "Reader of field `IRDSSEL`"]
pub type IRDSSEL_R = crate::R<bool, IRDSSEL_A>;
impl IRDSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDSSEL_A {
        match self.bits {
            false => IRDSSEL_A::IRDSSEL_0,
            true => IRDSSEL_A::IRDSSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRDSSEL_0`"]
    #[inline(always)]
    pub fn is_irdssel_0(&self) -> bool {
        *self == IRDSSEL_A::IRDSSEL_0
    }
    #[doc = "Checks if the value of the field is `IRDSSEL_1`"]
    #[inline(always)]
    pub fn is_irdssel_1(&self) -> bool {
        *self == IRDSSEL_A::IRDSSEL_1
    }
}
#[doc = "Write proxy for field `IRDSSEL`"]
pub struct IRDSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDSSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "From hardware peripherals upon device configuration"]
    #[inline(always)]
    pub fn irdssel_0(self) -> &'a mut W {
        self.variant(IRDSSEL_A::IRDSSEL_0)
    }
    #[doc = "From IRDATA bit"]
    #[inline(always)]
    pub fn irdssel_1(self) -> &'a mut W {
        self.variant(IRDSSEL_A::IRDSSEL_1)
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
#[doc = "Infrared data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDATA_A {
    #[doc = "0: Infrared data logic 0"]
    IRDATA_0,
    #[doc = "1: Infrared data logic 1"]
    IRDATA_1,
}
impl From<IRDATA_A> for bool {
    #[inline(always)]
    fn from(variant: IRDATA_A) -> Self {
        match variant {
            IRDATA_A::IRDATA_0 => false,
            IRDATA_A::IRDATA_1 => true,
        }
    }
}
#[doc = "Reader of field `IRDATA`"]
pub type IRDATA_R = crate::R<bool, IRDATA_A>;
impl IRDATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDATA_A {
        match self.bits {
            false => IRDATA_A::IRDATA_0,
            true => IRDATA_A::IRDATA_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRDATA_0`"]
    #[inline(always)]
    pub fn is_irdata_0(&self) -> bool {
        *self == IRDATA_A::IRDATA_0
    }
    #[doc = "Checks if the value of the field is `IRDATA_1`"]
    #[inline(always)]
    pub fn is_irdata_1(&self) -> bool {
        *self == IRDATA_A::IRDATA_1
    }
}
#[doc = "Write proxy for field `IRDATA`"]
pub struct IRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDATA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Infrared data logic 0"]
    #[inline(always)]
    pub fn irdata_0(self) -> &'a mut W {
        self.variant(IRDATA_A::IRDATA_0)
    }
    #[doc = "Infrared data logic 1"]
    #[inline(always)]
    pub fn irdata_1(self) -> &'a mut W {
        self.variant(IRDATA_A::IRDATA_1)
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
#[doc = "Captivate Conversion triggered Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCSEL_A {
    #[doc = "0: External source is selected"]
    SYNCSEL_0,
    #[doc = "1: ADC as the source is selected"]
    SYNCSEL_1,
    #[doc = "2: internal source is selected"]
    SYNCSEL_2,
    #[doc = "3: Reserved"]
    SYNCSEL_3,
}
impl From<SYNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSEL_A) -> Self {
        match variant {
            SYNCSEL_A::SYNCSEL_0 => 0,
            SYNCSEL_A::SYNCSEL_1 => 1,
            SYNCSEL_A::SYNCSEL_2 => 2,
            SYNCSEL_A::SYNCSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `SYNCSEL`"]
pub type SYNCSEL_R = crate::R<u8, SYNCSEL_A>;
impl SYNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSEL_A {
        match self.bits {
            0 => SYNCSEL_A::SYNCSEL_0,
            1 => SYNCSEL_A::SYNCSEL_1,
            2 => SYNCSEL_A::SYNCSEL_2,
            3 => SYNCSEL_A::SYNCSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_0`"]
    #[inline(always)]
    pub fn is_syncsel_0(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_0
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_1`"]
    #[inline(always)]
    pub fn is_syncsel_1(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_1
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_2`"]
    #[inline(always)]
    pub fn is_syncsel_2(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_2
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_3`"]
    #[inline(always)]
    pub fn is_syncsel_3(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_3
    }
}
#[doc = "Write proxy for field `SYNCSEL`"]
pub struct SYNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn syncsel_0(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_0)
    }
    #[doc = "ADC as the source is selected"]
    #[inline(always)]
    pub fn syncsel_1(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_1)
    }
    #[doc = "internal source is selected"]
    #[inline(always)]
    pub fn syncsel_2(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn syncsel_3(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IRPSEL_R {
        IRPSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IRMSEL_R {
        IRMSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IRDSSEL_R {
        IRDSSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&self) -> IRDATA_R {
        IRDATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&mut self) -> IRPSEL_W {
        IRPSEL_W { w: self }
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&mut self) -> IRMSEL_W {
        IRMSEL_W { w: self }
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&mut self) -> IRDSSEL_W {
        IRDSSEL_W { w: self }
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&mut self) -> IRDATA_W {
        IRDATA_W { w: self }
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W {
        SYNCSEL_W { w: self }
    }
}
