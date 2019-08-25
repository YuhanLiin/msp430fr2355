#[doc = "Reader of register UCA1IV"]
pub type R = crate::R<u16, super::UCA1IV>;
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCRXIFG,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCTXIFG,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCSTTIFG,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCTXCPTIFG,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        match variant {
            UCIV_A::NONE => 0,
            UCIV_A::UCRXIFG => 2,
            UCIV_A::UCTXIFG => 4,
            UCIV_A::UCSTTIFG => 6,
            UCIV_A::UCTXCPTIFG => 8,
        }
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
            6 => Val(UCIV_A::UCSTTIFG),
            8 => Val(UCIV_A::UCTXCPTIFG),
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
    #[doc = "Checks if the value of the field is `UCSTTIFG`"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == UCIV_A::UCSTTIFG
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG`"]
    #[inline(always)]
    pub fn is_uctxcptifg(&self) -> bool {
        *self == UCIV_A::UCTXCPTIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
