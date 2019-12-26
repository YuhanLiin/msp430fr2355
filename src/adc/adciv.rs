#[doc = "Reader of register ADCIV"]
pub type R = crate::R<u16, super::ADCIV>;
#[doc = "Writer for register ADCIV"]
pub type W = crate::W<u16, super::ADCIV>;
#[doc = "Register ADCIV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    ADCOVIFG = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    ADCTOVIFG = 4,
    #[doc = "6: Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    ADCHIIFG = 6,
    #[doc = "8: Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    ADCLOIFG = 8,
    #[doc = "10: nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    ADCINIFG = 10,
    #[doc = "12: Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    ADCIFG0 = 12,
}
impl From<ADCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: ADCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCIV`"]
pub type ADCIV_R = crate::R<u16, ADCIV_A>;
impl ADCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCIV_A::NONE),
            2 => Val(ADCIV_A::ADCOVIFG),
            4 => Val(ADCIV_A::ADCTOVIFG),
            6 => Val(ADCIV_A::ADCHIIFG),
            8 => Val(ADCIV_A::ADCLOIFG),
            10 => Val(ADCIV_A::ADCINIFG),
            12 => Val(ADCIV_A::ADCIFG0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG`"]
    #[inline(always)]
    pub fn is_adcovifg(&self) -> bool {
        *self == ADCIV_A::ADCOVIFG
    }
    #[doc = "Checks if the value of the field is `ADCTOVIFG`"]
    #[inline(always)]
    pub fn is_adctovifg(&self) -> bool {
        *self == ADCIV_A::ADCTOVIFG
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG`"]
    #[inline(always)]
    pub fn is_adchiifg(&self) -> bool {
        *self == ADCIV_A::ADCHIIFG
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG`"]
    #[inline(always)]
    pub fn is_adcloifg(&self) -> bool {
        *self == ADCIV_A::ADCLOIFG
    }
    #[doc = "Checks if the value of the field is `ADCINIFG`"]
    #[inline(always)]
    pub fn is_adcinifg(&self) -> bool {
        *self == ADCIV_A::ADCINIFG
    }
    #[doc = "Checks if the value of the field is `ADCIFG0`"]
    #[inline(always)]
    pub fn is_adcifg0(&self) -> bool {
        *self == ADCIV_A::ADCIFG0
    }
}
#[doc = "Write proxy for field `ADCIV`"]
pub struct ADCIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADCIV_A::NONE)
    }
    #[doc = "Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adcovifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCOVIFG)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn adctovifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCTOVIFG)
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn adchiifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCHIIFG)
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn adcloifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCLOIFG)
    }
    #[doc = "nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn adcinifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCINIFG)
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn adcifg0(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCIFG0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&self) -> ADCIV_R {
        ADCIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&mut self) -> ADCIV_W {
        ADCIV_W { w: self }
    }
}
