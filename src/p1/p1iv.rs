#[doc = "Register `P1IV` reader"]
pub type R = crate::R<P1ivSpec>;
#[doc = "Register `P1IV` writer"]
pub type W = crate::W<P1ivSpec>;
#[doc = "Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1ifg7 = 16,
}
impl From<P1iv> for u8 {
    #[inline(always)]
    fn from(variant: P1iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1iv {
    type Ux = u8;
}
impl crate::IsEnum for P1iv {}
#[doc = "Field `P1IV` reader - Port 1 interrupt vector value"]
pub type P1ivR = crate::FieldReader<P1iv>;
impl P1ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P1iv> {
        match self.bits {
            0 => Some(P1iv::None),
            2 => Some(P1iv::P1ifg0),
            4 => Some(P1iv::P1ifg1),
            6 => Some(P1iv::P1ifg2),
            8 => Some(P1iv::P1ifg3),
            10 => Some(P1iv::P1ifg4),
            12 => Some(P1iv::P1ifg5),
            14 => Some(P1iv::P1ifg6),
            16 => Some(P1iv::P1ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P1iv::None
    }
    #[doc = "Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p1ifg0(&self) -> bool {
        *self == P1iv::P1ifg0
    }
    #[doc = "Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    #[inline(always)]
    pub fn is_p1ifg1(&self) -> bool {
        *self == P1iv::P1ifg1
    }
    #[doc = "Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    #[inline(always)]
    pub fn is_p1ifg2(&self) -> bool {
        *self == P1iv::P1ifg2
    }
    #[doc = "Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    #[inline(always)]
    pub fn is_p1ifg3(&self) -> bool {
        *self == P1iv::P1ifg3
    }
    #[doc = "Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    #[inline(always)]
    pub fn is_p1ifg4(&self) -> bool {
        *self == P1iv::P1ifg4
    }
    #[doc = "Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    #[inline(always)]
    pub fn is_p1ifg5(&self) -> bool {
        *self == P1iv::P1ifg5
    }
    #[doc = "Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    #[inline(always)]
    pub fn is_p1ifg6(&self) -> bool {
        *self == P1iv::P1ifg6
    }
    #[doc = "Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p1ifg7(&self) -> bool {
        *self == P1iv::P1ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1ivR {
        P1ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ivSpec;
impl crate::RegisterSpec for P1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p1iv::R`](R) reader structure"]
impl crate::Readable for P1ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p1iv::W`](W) writer structure"]
impl crate::Writable for P1ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1ivSpec {}
