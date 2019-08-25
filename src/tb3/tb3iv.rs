#[doc = "Reader of register TB3IV"]
pub type R = crate::R<u16, super::TB3IV>;
#[doc = "Timer_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    TBCCR1,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    TBCCR2,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    TBCCR3,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    TBCCR4,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    TBCCR5,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    TBCCR6,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    TBIFG,
}
impl From<TBIV_A> for u16 {
    #[inline(always)]
    fn from(variant: TBIV_A) -> Self {
        match variant {
            TBIV_A::NONE => 0,
            TBIV_A::TBCCR1 => 2,
            TBIV_A::TBCCR2 => 4,
            TBIV_A::TBCCR3 => 6,
            TBIV_A::TBCCR4 => 8,
            TBIV_A::TBCCR5 => 10,
            TBIV_A::TBCCR6 => 12,
            TBIV_A::TBIFG => 14,
        }
    }
}
#[doc = "Reader of field `TBIV`"]
pub type TBIV_R = crate::R<u16, TBIV_A>;
impl TBIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TBIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TBIV_A::NONE),
            2 => Val(TBIV_A::TBCCR1),
            4 => Val(TBIV_A::TBCCR2),
            6 => Val(TBIV_A::TBCCR3),
            8 => Val(TBIV_A::TBCCR4),
            10 => Val(TBIV_A::TBCCR5),
            12 => Val(TBIV_A::TBCCR6),
            14 => Val(TBIV_A::TBIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TBIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TBCCR1`"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == TBIV_A::TBCCR1
    }
    #[doc = "Checks if the value of the field is `TBCCR2`"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == TBIV_A::TBCCR2
    }
    #[doc = "Checks if the value of the field is `TBCCR3`"]
    #[inline(always)]
    pub fn is_tbccr3(&self) -> bool {
        *self == TBIV_A::TBCCR3
    }
    #[doc = "Checks if the value of the field is `TBCCR4`"]
    #[inline(always)]
    pub fn is_tbccr4(&self) -> bool {
        *self == TBIV_A::TBCCR4
    }
    #[doc = "Checks if the value of the field is `TBCCR5`"]
    #[inline(always)]
    pub fn is_tbccr5(&self) -> bool {
        *self == TBIV_A::TBCCR5
    }
    #[doc = "Checks if the value of the field is `TBCCR6`"]
    #[inline(always)]
    pub fn is_tbccr6(&self) -> bool {
        *self == TBIV_A::TBCCR6
    }
    #[doc = "Checks if the value of the field is `TBIFG`"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == TBIV_A::TBIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer_B interrupt vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TBIV_R {
        TBIV_R::new((self.bits & 0xffff) as u16)
    }
}
