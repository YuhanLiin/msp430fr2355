#[doc = "Reader of register P7IV"]
pub type R = crate::R<u16, super::P7IV>;
#[doc = "Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P7IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7IFG0,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7IFG1,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7IFG2,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7IFG3,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7IFG4,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7IFG5,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7IFG6,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7IFG7,
}
impl From<P7IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P7IV_A) -> Self {
        match variant {
            P7IV_A::NONE => 0,
            P7IV_A::P7IFG0 => 2,
            P7IV_A::P7IFG1 => 4,
            P7IV_A::P7IFG2 => 6,
            P7IV_A::P7IFG3 => 8,
            P7IV_A::P7IFG4 => 10,
            P7IV_A::P7IFG5 => 12,
            P7IV_A::P7IFG6 => 14,
            P7IV_A::P7IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P7IV`"]
pub type P7IV_R = crate::R<u8, P7IV_A>;
impl P7IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P7IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P7IV_A::NONE),
            2 => Val(P7IV_A::P7IFG0),
            4 => Val(P7IV_A::P7IFG1),
            6 => Val(P7IV_A::P7IFG2),
            8 => Val(P7IV_A::P7IFG3),
            10 => Val(P7IV_A::P7IFG4),
            12 => Val(P7IV_A::P7IFG5),
            14 => Val(P7IV_A::P7IFG6),
            16 => Val(P7IV_A::P7IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P7IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P7IFG0`"]
    #[inline(always)]
    pub fn is_p7ifg0(&self) -> bool {
        *self == P7IV_A::P7IFG0
    }
    #[doc = "Checks if the value of the field is `P7IFG1`"]
    #[inline(always)]
    pub fn is_p7ifg1(&self) -> bool {
        *self == P7IV_A::P7IFG1
    }
    #[doc = "Checks if the value of the field is `P7IFG2`"]
    #[inline(always)]
    pub fn is_p7ifg2(&self) -> bool {
        *self == P7IV_A::P7IFG2
    }
    #[doc = "Checks if the value of the field is `P7IFG3`"]
    #[inline(always)]
    pub fn is_p7ifg3(&self) -> bool {
        *self == P7IV_A::P7IFG3
    }
    #[doc = "Checks if the value of the field is `P7IFG4`"]
    #[inline(always)]
    pub fn is_p7ifg4(&self) -> bool {
        *self == P7IV_A::P7IFG4
    }
    #[doc = "Checks if the value of the field is `P7IFG5`"]
    #[inline(always)]
    pub fn is_p7ifg5(&self) -> bool {
        *self == P7IV_A::P7IFG5
    }
    #[doc = "Checks if the value of the field is `P7IFG6`"]
    #[inline(always)]
    pub fn is_p7ifg6(&self) -> bool {
        *self == P7IV_A::P7IFG6
    }
    #[doc = "Checks if the value of the field is `P7IFG7`"]
    #[inline(always)]
    pub fn is_p7ifg7(&self) -> bool {
        *self == P7IV_A::P7IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7IV_R {
        P7IV_R::new((self.bits & 0x1f) as u8)
    }
}
