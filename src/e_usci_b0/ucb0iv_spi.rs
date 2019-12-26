#[doc = "Reader of register UCB0IV_SPI"]
pub type R = crate::R<u16, super::UCB0IV_SPI>;
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCRXIFG = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    UCTXIFG = 4,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCIV`"]
pub type UCIV_R = crate::R<u16, UCIV_A>;
impl UCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UCIV_A::NONE),
            2 => Val(UCIV_A::UCRXIFG),
            4 => Val(UCIV_A::UCTXIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCRXIFG`"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == UCIV_A::UCRXIFG
    }
    #[doc = "Checks if the value of the field is `UCTXIFG`"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == UCIV_A::UCTXIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
