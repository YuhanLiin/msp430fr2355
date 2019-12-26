#[doc = "Reader of register UCB0IV"]
pub type R = crate::R<u16, super::UCB0IV>;
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    UCALIFG = 2,
    #[doc = "4: Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    UCNACKIFG = 4,
    #[doc = "6: Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    UCSTTIFG = 6,
    #[doc = "8: Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    UCSTPIFG = 8,
    #[doc = "10: Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    UCRXIFG3 = 10,
    #[doc = "12: Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    UCTXIFG3 = 12,
    #[doc = "14: Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    UCRXIFG2 = 14,
    #[doc = "16: Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    UCTXIFG2 = 16,
    #[doc = "18: Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    UCRXIFG1 = 18,
    #[doc = "20: Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    UCTXIFG1 = 20,
    #[doc = "22: Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    UCRXIFG0 = 22,
    #[doc = "24: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    UCTXIFG0 = 24,
    #[doc = "26: Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    UCBCNTIFG = 26,
    #[doc = "28: Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    UCCLTOIFG = 28,
    #[doc = "30: Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    UCBIT9IFG = 30,
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
            2 => Val(UCIV_A::UCALIFG),
            4 => Val(UCIV_A::UCNACKIFG),
            6 => Val(UCIV_A::UCSTTIFG),
            8 => Val(UCIV_A::UCSTPIFG),
            10 => Val(UCIV_A::UCRXIFG3),
            12 => Val(UCIV_A::UCTXIFG3),
            14 => Val(UCIV_A::UCRXIFG2),
            16 => Val(UCIV_A::UCTXIFG2),
            18 => Val(UCIV_A::UCRXIFG1),
            20 => Val(UCIV_A::UCTXIFG1),
            22 => Val(UCIV_A::UCRXIFG0),
            24 => Val(UCIV_A::UCTXIFG0),
            26 => Val(UCIV_A::UCBCNTIFG),
            28 => Val(UCIV_A::UCCLTOIFG),
            30 => Val(UCIV_A::UCBIT9IFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCALIFG`"]
    #[inline(always)]
    pub fn is_ucalifg(&self) -> bool {
        *self == UCIV_A::UCALIFG
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG`"]
    #[inline(always)]
    pub fn is_ucnackifg(&self) -> bool {
        *self == UCIV_A::UCNACKIFG
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG`"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == UCIV_A::UCSTTIFG
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG`"]
    #[inline(always)]
    pub fn is_ucstpifg(&self) -> bool {
        *self == UCIV_A::UCSTPIFG
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3`"]
    #[inline(always)]
    pub fn is_ucrxifg3(&self) -> bool {
        *self == UCIV_A::UCRXIFG3
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3`"]
    #[inline(always)]
    pub fn is_uctxifg3(&self) -> bool {
        *self == UCIV_A::UCTXIFG3
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2`"]
    #[inline(always)]
    pub fn is_ucrxifg2(&self) -> bool {
        *self == UCIV_A::UCRXIFG2
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2`"]
    #[inline(always)]
    pub fn is_uctxifg2(&self) -> bool {
        *self == UCIV_A::UCTXIFG2
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1`"]
    #[inline(always)]
    pub fn is_ucrxifg1(&self) -> bool {
        *self == UCIV_A::UCRXIFG1
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1`"]
    #[inline(always)]
    pub fn is_uctxifg1(&self) -> bool {
        *self == UCIV_A::UCTXIFG1
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0`"]
    #[inline(always)]
    pub fn is_ucrxifg0(&self) -> bool {
        *self == UCIV_A::UCRXIFG0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0`"]
    #[inline(always)]
    pub fn is_uctxifg0(&self) -> bool {
        *self == UCIV_A::UCTXIFG0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG`"]
    #[inline(always)]
    pub fn is_ucbcntifg(&self) -> bool {
        *self == UCIV_A::UCBCNTIFG
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG`"]
    #[inline(always)]
    pub fn is_uccltoifg(&self) -> bool {
        *self == UCIV_A::UCCLTOIFG
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG`"]
    #[inline(always)]
    pub fn is_ucbit9ifg(&self) -> bool {
        *self == UCIV_A::UCBIT9IFG
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
