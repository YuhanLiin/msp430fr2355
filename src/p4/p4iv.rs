#[doc = "Reader of register P4IV"]
pub type R = crate::R<u16, super::P4IV>;
#[doc = "Port 4 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4IFG7 = 16,
}
impl From<P4IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P4IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4IV`"]
pub type P4IV_R = crate::R<u8, P4IV_A>;
impl P4IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P4IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P4IV_A::NONE),
            2 => Val(P4IV_A::P4IFG0),
            4 => Val(P4IV_A::P4IFG1),
            6 => Val(P4IV_A::P4IFG2),
            8 => Val(P4IV_A::P4IFG3),
            10 => Val(P4IV_A::P4IFG4),
            12 => Val(P4IV_A::P4IFG5),
            14 => Val(P4IV_A::P4IFG6),
            16 => Val(P4IV_A::P4IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P4IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P4IFG0`"]
    #[inline(always)]
    pub fn is_p4ifg0(&self) -> bool {
        *self == P4IV_A::P4IFG0
    }
    #[doc = "Checks if the value of the field is `P4IFG1`"]
    #[inline(always)]
    pub fn is_p4ifg1(&self) -> bool {
        *self == P4IV_A::P4IFG1
    }
    #[doc = "Checks if the value of the field is `P4IFG2`"]
    #[inline(always)]
    pub fn is_p4ifg2(&self) -> bool {
        *self == P4IV_A::P4IFG2
    }
    #[doc = "Checks if the value of the field is `P4IFG3`"]
    #[inline(always)]
    pub fn is_p4ifg3(&self) -> bool {
        *self == P4IV_A::P4IFG3
    }
    #[doc = "Checks if the value of the field is `P4IFG4`"]
    #[inline(always)]
    pub fn is_p4ifg4(&self) -> bool {
        *self == P4IV_A::P4IFG4
    }
    #[doc = "Checks if the value of the field is `P4IFG5`"]
    #[inline(always)]
    pub fn is_p4ifg5(&self) -> bool {
        *self == P4IV_A::P4IFG5
    }
    #[doc = "Checks if the value of the field is `P4IFG6`"]
    #[inline(always)]
    pub fn is_p4ifg6(&self) -> bool {
        *self == P4IV_A::P4IFG6
    }
    #[doc = "Checks if the value of the field is `P4IFG7`"]
    #[inline(always)]
    pub fn is_p4ifg7(&self) -> bool {
        *self == P4IV_A::P4IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4IV_R {
        P4IV_R::new((self.bits & 0x1f) as u8)
    }
}
