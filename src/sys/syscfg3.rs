#[doc = "Reader of register SYSCFG3"]
pub type R = crate::R<u16, super::SYSCFG3>;
#[doc = "Writer for register SYSCFG3"]
pub type W = crate::W<u16, super::SYSCFG3>;
#[doc = "Register SYSCFG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "eUSCIA remapping source selection, please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCIARMP_A {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    USCIARMP_0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    USCIARMP_1,
}
impl From<USCIARMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIARMP_A) -> Self {
        match variant {
            USCIARMP_A::USCIARMP_0 => false,
            USCIARMP_A::USCIARMP_1 => true,
        }
    }
}
#[doc = "Reader of field `USCIARMP`"]
pub type USCIARMP_R = crate::R<bool, USCIARMP_A>;
impl USCIARMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIARMP_A {
        match self.bits {
            false => USCIARMP_A::USCIARMP_0,
            true => USCIARMP_A::USCIARMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIARMP_0`"]
    #[inline(always)]
    pub fn is_usciarmp_0(&self) -> bool {
        *self == USCIARMP_A::USCIARMP_0
    }
    #[doc = "Checks if the value of the field is `USCIARMP_1`"]
    #[inline(always)]
    pub fn is_usciarmp_1(&self) -> bool {
        *self == USCIARMP_A::USCIARMP_1
    }
}
#[doc = "Write proxy for field `USCIARMP`"]
pub struct USCIARMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USCIARMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USCIARMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_0(self) -> &'a mut W {
        self.variant(USCIARMP_A::USCIARMP_0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_1(self) -> &'a mut W {
        self.variant(USCIARMP_A::USCIARMP_1)
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
impl R {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&self) -> USCIARMP_R {
        USCIARMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&mut self) -> USCIARMP_W {
        USCIARMP_W { w: self }
    }
}
