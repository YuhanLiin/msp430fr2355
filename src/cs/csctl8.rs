#[doc = "Reader of register CSCTL8"]
pub type R = crate::R<u16, super::CSCTL8>;
#[doc = "Writer for register CSCTL8"]
pub type W = crate::W<u16, super::CSCTL8>;
#[doc = "Register CSCTL8 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL8 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ACLK clock request enable. Setting this enables conditional module requests for ACLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACLKREQEN_A {
    #[doc = "0: ACLK conditional requests are disabled."]
    ACLKREQEN_0 = 0,
    #[doc = "1: ACLK conditional requests are enabled."]
    ACLKREQEN_1 = 1,
}
impl From<ACLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACLKREQEN`"]
pub type ACLKREQEN_R = crate::R<bool, ACLKREQEN_A>;
impl ACLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLKREQEN_A {
        match self.bits {
            false => ACLKREQEN_A::ACLKREQEN_0,
            true => ACLKREQEN_A::ACLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACLKREQEN_0`"]
    #[inline(always)]
    pub fn is_aclkreqen_0(&self) -> bool {
        *self == ACLKREQEN_A::ACLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `ACLKREQEN_1`"]
    #[inline(always)]
    pub fn is_aclkreqen_1(&self) -> bool {
        *self == ACLKREQEN_A::ACLKREQEN_1
    }
}
#[doc = "Write proxy for field `ACLKREQEN`"]
pub struct ACLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACLK conditional requests are disabled."]
    #[inline(always)]
    pub fn aclkreqen_0(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::ACLKREQEN_0)
    }
    #[doc = "ACLK conditional requests are enabled."]
    #[inline(always)]
    pub fn aclkreqen_1(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::ACLKREQEN_1)
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
#[doc = "MCLK clock request enable. Setting this enables conditional module requests for MCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKREQEN_A {
    #[doc = "0: MCLK conditional requests are disabled."]
    MCLKREQEN_0 = 0,
    #[doc = "1: MCLK conditional requests are enabled."]
    MCLKREQEN_1 = 1,
}
impl From<MCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLKREQEN`"]
pub type MCLKREQEN_R = crate::R<bool, MCLKREQEN_A>;
impl MCLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKREQEN_A {
        match self.bits {
            false => MCLKREQEN_A::MCLKREQEN_0,
            true => MCLKREQEN_A::MCLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLKREQEN_0`"]
    #[inline(always)]
    pub fn is_mclkreqen_0(&self) -> bool {
        *self == MCLKREQEN_A::MCLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `MCLKREQEN_1`"]
    #[inline(always)]
    pub fn is_mclkreqen_1(&self) -> bool {
        *self == MCLKREQEN_A::MCLKREQEN_1
    }
}
#[doc = "Write proxy for field `MCLKREQEN`"]
pub struct MCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn mclkreqen_0(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::MCLKREQEN_0)
    }
    #[doc = "MCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn mclkreqen_1(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::MCLKREQEN_1)
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
#[doc = "SMCLK clock request enable. Setting this enables conditional module requests for SMCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLKREQEN_A {
    #[doc = "0: SMCLK conditional requests are disabled."]
    SMCLKREQEN_0 = 0,
    #[doc = "1: SMCLK conditional requests are enabled."]
    SMCLKREQEN_1 = 1,
}
impl From<SMCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLKREQEN`"]
pub type SMCLKREQEN_R = crate::R<bool, SMCLKREQEN_A>;
impl SMCLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKREQEN_A {
        match self.bits {
            false => SMCLKREQEN_A::SMCLKREQEN_0,
            true => SMCLKREQEN_A::SMCLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLKREQEN_0`"]
    #[inline(always)]
    pub fn is_smclkreqen_0(&self) -> bool {
        *self == SMCLKREQEN_A::SMCLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `SMCLKREQEN_1`"]
    #[inline(always)]
    pub fn is_smclkreqen_1(&self) -> bool {
        *self == SMCLKREQEN_A::SMCLKREQEN_1
    }
}
#[doc = "Write proxy for field `SMCLKREQEN`"]
pub struct SMCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMCLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn smclkreqen_0(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::SMCLKREQEN_0)
    }
    #[doc = "SMCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn smclkreqen_1(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::SMCLKREQEN_1)
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
#[doc = "MODOSC clock request enable. Setting this enables conditional module requests for MODOSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODOSCREQEN_A {
    #[doc = "0: MODOSC conditional requests are disabled."]
    MODOSCREQEN_0 = 0,
    #[doc = "1: MODOSC conditional requests are enabled."]
    MODOSCREQEN_1 = 1,
}
impl From<MODOSCREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODOSCREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODOSCREQEN`"]
pub type MODOSCREQEN_R = crate::R<bool, MODOSCREQEN_A>;
impl MODOSCREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODOSCREQEN_A {
        match self.bits {
            false => MODOSCREQEN_A::MODOSCREQEN_0,
            true => MODOSCREQEN_A::MODOSCREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODOSCREQEN_0`"]
    #[inline(always)]
    pub fn is_modoscreqen_0(&self) -> bool {
        *self == MODOSCREQEN_A::MODOSCREQEN_0
    }
    #[doc = "Checks if the value of the field is `MODOSCREQEN_1`"]
    #[inline(always)]
    pub fn is_modoscreqen_1(&self) -> bool {
        *self == MODOSCREQEN_A::MODOSCREQEN_1
    }
}
#[doc = "Write proxy for field `MODOSCREQEN`"]
pub struct MODOSCREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODOSCREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODOSCREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MODOSC conditional requests are disabled."]
    #[inline(always)]
    pub fn modoscreqen_0(self) -> &'a mut W {
        self.variant(MODOSCREQEN_A::MODOSCREQEN_0)
    }
    #[doc = "MODOSC conditional requests are enabled."]
    #[inline(always)]
    pub fn modoscreqen_1(self) -> &'a mut W {
        self.variant(MODOSCREQEN_A::MODOSCREQEN_1)
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
impl R {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W {
        ACLKREQEN_W { w: self }
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W {
        MCLKREQEN_W { w: self }
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W {
        SMCLKREQEN_W { w: self }
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W {
        MODOSCREQEN_W { w: self }
    }
}
