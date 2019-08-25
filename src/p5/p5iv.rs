#[doc = "Reader of register P5IV"]
pub type R = crate::R<u16, super::P5IV>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5IFG0,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5IFG1,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5IFG2,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5IFG3,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5IFG4,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5IFG5,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5IFG6,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5IFG7,
}
impl From<P5IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P5IV_A) -> Self {
        match variant {
            P5IV_A::NONE => 0,
            P5IV_A::P5IFG0 => 2,
            P5IV_A::P5IFG1 => 4,
            P5IV_A::P5IFG2 => 6,
            P5IV_A::P5IFG3 => 8,
            P5IV_A::P5IFG4 => 10,
            P5IV_A::P5IFG5 => 12,
            P5IV_A::P5IFG6 => 14,
            P5IV_A::P5IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P5IV`"]
pub type P5IV_R = crate::R<u8, P5IV_A>;
impl P5IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P5IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P5IV_A::NONE),
            2 => Val(P5IV_A::P5IFG0),
            4 => Val(P5IV_A::P5IFG1),
            6 => Val(P5IV_A::P5IFG2),
            8 => Val(P5IV_A::P5IFG3),
            10 => Val(P5IV_A::P5IFG4),
            12 => Val(P5IV_A::P5IFG5),
            14 => Val(P5IV_A::P5IFG6),
            16 => Val(P5IV_A::P5IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P5IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P5IFG0`"]
    #[inline(always)]
    pub fn is_p5ifg0(&self) -> bool {
        *self == P5IV_A::P5IFG0
    }
    #[doc = "Checks if the value of the field is `P5IFG1`"]
    #[inline(always)]
    pub fn is_p5ifg1(&self) -> bool {
        *self == P5IV_A::P5IFG1
    }
    #[doc = "Checks if the value of the field is `P5IFG2`"]
    #[inline(always)]
    pub fn is_p5ifg2(&self) -> bool {
        *self == P5IV_A::P5IFG2
    }
    #[doc = "Checks if the value of the field is `P5IFG3`"]
    #[inline(always)]
    pub fn is_p5ifg3(&self) -> bool {
        *self == P5IV_A::P5IFG3
    }
    #[doc = "Checks if the value of the field is `P5IFG4`"]
    #[inline(always)]
    pub fn is_p5ifg4(&self) -> bool {
        *self == P5IV_A::P5IFG4
    }
    #[doc = "Checks if the value of the field is `P5IFG5`"]
    #[inline(always)]
    pub fn is_p5ifg5(&self) -> bool {
        *self == P5IV_A::P5IFG5
    }
    #[doc = "Checks if the value of the field is `P5IFG6`"]
    #[inline(always)]
    pub fn is_p5ifg6(&self) -> bool {
        *self == P5IV_A::P5IFG6
    }
    #[doc = "Checks if the value of the field is `P5IFG7`"]
    #[inline(always)]
    pub fn is_p5ifg7(&self) -> bool {
        *self == P5IV_A::P5IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5IV_R {
        P5IV_R::new((self.bits & 0x1f) as u8)
    }
}
