#[doc = "Register `P2IV` reader"]
pub type R = crate::R<P2ivSpec>;
#[doc = "Register `P2IV` writer"]
pub type W = crate::W<P2ivSpec>;
#[doc = "Port 2 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    P2ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    P2ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    P2ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    P2ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    P2ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    P2ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    P2ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    P2ifg7 = 16,
}
impl From<P2iv> for u8 {
    #[inline(always)]
    fn from(variant: P2iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2iv {
    type Ux = u8;
}
impl crate::IsEnum for P2iv {}
#[doc = "Field `P2IV` reader - Port 2 interrupt vector value"]
pub type P2ivR = crate::FieldReader<P2iv>;
impl P2ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P2iv> {
        match self.bits {
            0 => Some(P2iv::None),
            2 => Some(P2iv::P2ifg0),
            4 => Some(P2iv::P2ifg1),
            6 => Some(P2iv::P2ifg2),
            8 => Some(P2iv::P2ifg3),
            10 => Some(P2iv::P2ifg4),
            12 => Some(P2iv::P2ifg5),
            14 => Some(P2iv::P2ifg6),
            16 => Some(P2iv::P2ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P2iv::None
    }
    #[doc = "Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p2ifg0(&self) -> bool {
        *self == P2iv::P2ifg0
    }
    #[doc = "Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    #[inline(always)]
    pub fn is_p2ifg1(&self) -> bool {
        *self == P2iv::P2ifg1
    }
    #[doc = "Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    #[inline(always)]
    pub fn is_p2ifg2(&self) -> bool {
        *self == P2iv::P2ifg2
    }
    #[doc = "Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    #[inline(always)]
    pub fn is_p2ifg3(&self) -> bool {
        *self == P2iv::P2ifg3
    }
    #[doc = "Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    #[inline(always)]
    pub fn is_p2ifg4(&self) -> bool {
        *self == P2iv::P2ifg4
    }
    #[doc = "Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    #[inline(always)]
    pub fn is_p2ifg5(&self) -> bool {
        *self == P2iv::P2ifg5
    }
    #[doc = "Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    #[inline(always)]
    pub fn is_p2ifg6(&self) -> bool {
        *self == P2iv::P2ifg6
    }
    #[doc = "Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p2ifg7(&self) -> bool {
        *self == P2iv::P2ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&self) -> P2ivR {
        P2ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 2 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ivSpec;
impl crate::RegisterSpec for P2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p2iv::R`](R) reader structure"]
impl crate::Readable for P2ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p2iv::W`](W) writer structure"]
impl crate::Writable for P2ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2IV to value 0"]
impl crate::Resettable for P2ivSpec {}
