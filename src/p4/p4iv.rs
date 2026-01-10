#[doc = "Register `P4IV` reader"]
pub type R = crate::R<P4ivSpec>;
#[doc = "Register `P4IV` writer"]
pub type W = crate::W<P4ivSpec>;
#[doc = "Port 4 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4ifg7 = 16,
}
impl From<P4iv> for u8 {
    #[inline(always)]
    fn from(variant: P4iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P4iv {
    type Ux = u8;
}
impl crate::IsEnum for P4iv {}
#[doc = "Field `P4IV` reader - Port 4 interrupt vector value"]
pub type P4ivR = crate::FieldReader<P4iv>;
impl P4ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P4iv> {
        match self.bits {
            0 => Some(P4iv::None),
            2 => Some(P4iv::P4ifg0),
            4 => Some(P4iv::P4ifg1),
            6 => Some(P4iv::P4ifg2),
            8 => Some(P4iv::P4ifg3),
            10 => Some(P4iv::P4ifg4),
            12 => Some(P4iv::P4ifg5),
            14 => Some(P4iv::P4ifg6),
            16 => Some(P4iv::P4ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P4iv::None
    }
    #[doc = "Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p4ifg0(&self) -> bool {
        *self == P4iv::P4ifg0
    }
    #[doc = "Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    #[inline(always)]
    pub fn is_p4ifg1(&self) -> bool {
        *self == P4iv::P4ifg1
    }
    #[doc = "Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    #[inline(always)]
    pub fn is_p4ifg2(&self) -> bool {
        *self == P4iv::P4ifg2
    }
    #[doc = "Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    #[inline(always)]
    pub fn is_p4ifg3(&self) -> bool {
        *self == P4iv::P4ifg3
    }
    #[doc = "Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    #[inline(always)]
    pub fn is_p4ifg4(&self) -> bool {
        *self == P4iv::P4ifg4
    }
    #[doc = "Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    #[inline(always)]
    pub fn is_p4ifg5(&self) -> bool {
        *self == P4iv::P4ifg5
    }
    #[doc = "Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    #[inline(always)]
    pub fn is_p4ifg6(&self) -> bool {
        *self == P4iv::P4ifg6
    }
    #[doc = "Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p4ifg7(&self) -> bool {
        *self == P4iv::P4ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4ivR {
        P4ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 4 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p4iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4ivSpec;
impl crate::RegisterSpec for P4ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p4iv::R`](R) reader structure"]
impl crate::Readable for P4ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p4iv::W`](W) writer structure"]
impl crate::Writable for P4ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4IV to value 0"]
impl crate::Resettable for P4ivSpec {}
