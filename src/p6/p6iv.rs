#[doc = "Reader of register P6IV"]
pub type R = crate::R<u16, super::P6IV>;
#[doc = "Port 6 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    P6IFG0,
    #[doc = "4: Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    P6IFG1,
    #[doc = "6: Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    P6IFG2,
    #[doc = "8: Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    P6IFG3,
    #[doc = "10: Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    P6IFG4,
    #[doc = "12: Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    P6IFG5,
    #[doc = "14: Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    P6IFG6,
    #[doc = "16: Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    P6IFG7,
}
impl From<P6IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P6IV_A) -> Self {
        match variant {
            P6IV_A::NONE => 0,
            P6IV_A::P6IFG0 => 2,
            P6IV_A::P6IFG1 => 4,
            P6IV_A::P6IFG2 => 6,
            P6IV_A::P6IFG3 => 8,
            P6IV_A::P6IFG4 => 10,
            P6IV_A::P6IFG5 => 12,
            P6IV_A::P6IFG6 => 14,
            P6IV_A::P6IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P6IV`"]
pub type P6IV_R = crate::R<u8, P6IV_A>;
impl P6IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P6IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P6IV_A::NONE),
            2 => Val(P6IV_A::P6IFG0),
            4 => Val(P6IV_A::P6IFG1),
            6 => Val(P6IV_A::P6IFG2),
            8 => Val(P6IV_A::P6IFG3),
            10 => Val(P6IV_A::P6IFG4),
            12 => Val(P6IV_A::P6IFG5),
            14 => Val(P6IV_A::P6IFG6),
            16 => Val(P6IV_A::P6IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P6IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P6IFG0`"]
    #[inline(always)]
    pub fn is_p6ifg0(&self) -> bool {
        *self == P6IV_A::P6IFG0
    }
    #[doc = "Checks if the value of the field is `P6IFG1`"]
    #[inline(always)]
    pub fn is_p6ifg1(&self) -> bool {
        *self == P6IV_A::P6IFG1
    }
    #[doc = "Checks if the value of the field is `P6IFG2`"]
    #[inline(always)]
    pub fn is_p6ifg2(&self) -> bool {
        *self == P6IV_A::P6IFG2
    }
    #[doc = "Checks if the value of the field is `P6IFG3`"]
    #[inline(always)]
    pub fn is_p6ifg3(&self) -> bool {
        *self == P6IV_A::P6IFG3
    }
    #[doc = "Checks if the value of the field is `P6IFG4`"]
    #[inline(always)]
    pub fn is_p6ifg4(&self) -> bool {
        *self == P6IV_A::P6IFG4
    }
    #[doc = "Checks if the value of the field is `P6IFG5`"]
    #[inline(always)]
    pub fn is_p6ifg5(&self) -> bool {
        *self == P6IV_A::P6IFG5
    }
    #[doc = "Checks if the value of the field is `P6IFG6`"]
    #[inline(always)]
    pub fn is_p6ifg6(&self) -> bool {
        *self == P6IV_A::P6IFG6
    }
    #[doc = "Checks if the value of the field is `P6IFG7`"]
    #[inline(always)]
    pub fn is_p6ifg7(&self) -> bool {
        *self == P6IV_A::P6IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6IV_R {
        P6IV_R::new((self.bits & 0x1f) as u8)
    }
}
