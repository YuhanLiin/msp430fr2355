#[doc = "Reader of register CP1INT"]
pub type R = crate::R<u16, super::CP1INT>;
#[doc = "Writer for register CP1INT"]
pub type W = crate::W<u16, super::CP1INT>;
#[doc = "Register CP1INT `reset()`'s with value 0"]
impl crate::ResetValue for super::CP1INT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIFG_A {
    #[doc = "0: No interrupt pending."]
    CPIFG_0,
    #[doc = "1: Output interrupt pending."]
    CPIFG_1,
}
impl From<CPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CPIFG_A) -> Self {
        match variant {
            CPIFG_A::CPIFG_0 => false,
            CPIFG_A::CPIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `CPIFG`"]
pub type CPIFG_R = crate::R<bool, CPIFG_A>;
impl CPIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIFG_A {
        match self.bits {
            false => CPIFG_A::CPIFG_0,
            true => CPIFG_A::CPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIFG_0`"]
    #[inline(always)]
    pub fn is_cpifg_0(&self) -> bool {
        *self == CPIFG_A::CPIFG_0
    }
    #[doc = "Checks if the value of the field is `CPIFG_1`"]
    #[inline(always)]
    pub fn is_cpifg_1(&self) -> bool {
        *self == CPIFG_A::CPIFG_1
    }
}
#[doc = "Write proxy for field `CPIFG`"]
pub struct CPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpifg_0(self) -> &'a mut W {
        self.variant(CPIFG_A::CPIFG_0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpifg_1(self) -> &'a mut W {
        self.variant(CPIFG_A::CPIFG_1)
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
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIIFG_A {
    #[doc = "0: No interrupt pending."]
    CPIIFG_0,
    #[doc = "1: Output interrupt pending."]
    CPIIFG_1,
}
impl From<CPIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CPIIFG_A) -> Self {
        match variant {
            CPIIFG_A::CPIIFG_0 => false,
            CPIIFG_A::CPIIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `CPIIFG`"]
pub type CPIIFG_R = crate::R<bool, CPIIFG_A>;
impl CPIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIIFG_A {
        match self.bits {
            false => CPIIFG_A::CPIIFG_0,
            true => CPIIFG_A::CPIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIIFG_0`"]
    #[inline(always)]
    pub fn is_cpiifg_0(&self) -> bool {
        *self == CPIIFG_A::CPIIFG_0
    }
    #[doc = "Checks if the value of the field is `CPIIFG_1`"]
    #[inline(always)]
    pub fn is_cpiifg_1(&self) -> bool {
        *self == CPIIFG_A::CPIIFG_1
    }
}
#[doc = "Write proxy for field `CPIIFG`"]
pub struct CPIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_0(self) -> &'a mut W {
        self.variant(CPIIFG_A::CPIIFG_0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_1(self) -> &'a mut W {
        self.variant(CPIIFG_A::CPIIFG_1)
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
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&self) -> CPIFG_R {
        CPIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&self) -> CPIIFG_R {
        CPIIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&mut self) -> CPIFG_W {
        CPIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&mut self) -> CPIIFG_W {
        CPIIFG_W { w: self }
    }
}
