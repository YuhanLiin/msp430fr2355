#[doc = "Register `TB0CTL` reader"]
pub type R = crate::R<Tb0ctlSpec>;
#[doc = "Register `TB0CTL` writer"]
pub type W = crate::W<Tb0ctlSpec>;
#[doc = "TimerB interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbifg {
    #[doc = "0: No interrupt pending"]
    Tbifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Tbifg1 = 1,
}
impl From<Tbifg> for bool {
    #[inline(always)]
    fn from(variant: Tbifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIFG` reader - TimerB interrupt flag"]
pub type TbifgR = crate::BitReader<Tbifg>;
impl TbifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbifg {
        match self.bits {
            false => Tbifg::Tbifg0,
            true => Tbifg::Tbifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_tbifg_0(&self) -> bool {
        *self == Tbifg::Tbifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_tbifg_1(&self) -> bool {
        *self == Tbifg::Tbifg1
    }
}
#[doc = "Field `TBIFG` writer - TimerB interrupt flag"]
pub type TbifgW<'a, REG> = crate::BitWriter<'a, REG, Tbifg>;
impl<'a, REG> TbifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn tbifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tbifg::Tbifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn tbifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbifg::Tbifg1)
    }
}
#[doc = "TimerB interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbie {
    #[doc = "0: Interrupt disabled"]
    Tbie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Tbie1 = 1,
}
impl From<Tbie> for bool {
    #[inline(always)]
    fn from(variant: Tbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBIE` reader - TimerB interrupt enable"]
pub type TbieR = crate::BitReader<Tbie>;
impl TbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbie {
        match self.bits {
            false => Tbie::Tbie0,
            true => Tbie::Tbie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_tbie_0(&self) -> bool {
        *self == Tbie::Tbie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_tbie_1(&self) -> bool {
        *self == Tbie::Tbie1
    }
}
#[doc = "Field `TBIE` writer - TimerB interrupt enable"]
pub type TbieW<'a, REG> = crate::BitWriter<'a, REG, Tbie>;
impl<'a, REG> TbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn tbie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tbie::Tbie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn tbie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbie::Tbie1)
    }
}
#[doc = "Field `TBCLR` reader - TimerB clear"]
pub type TbclrR = crate::BitReader;
#[doc = "Field `TBCLR` writer - TimerB clear"]
pub type TbclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mc {
    #[doc = "0: Stop mode: Timer is halted"]
    Stop = 0,
    #[doc = "1: Up mode: Timer counts up to TBxCL0"]
    Up = 1,
    #[doc = "2: Continuous mode: Timer counts up to the value set by CNTL"]
    Continuous = 2,
    #[doc = "3: Up/down mode: Timer counts up to TBxCL0 then down to 0000h"]
    Updown = 3,
}
impl From<Mc> for u8 {
    #[inline(always)]
    fn from(variant: Mc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mc {
    type Ux = u8;
}
impl crate::IsEnum for Mc {}
#[doc = "Field `MC` reader - Mode control"]
pub type McR = crate::FieldReader<Mc>;
impl McR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mc {
        match self.bits {
            0 => Mc::Stop,
            1 => Mc::Up,
            2 => Mc::Continuous,
            3 => Mc::Updown,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mc::Stop
    }
    #[doc = "Up mode: Timer counts up to TBxCL0"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mc::Up
    }
    #[doc = "Continuous mode: Timer counts up to the value set by CNTL"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Mc::Continuous
    }
    #[doc = "Up/down mode: Timer counts up to TBxCL0 then down to 0000h"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Mc::Updown
    }
}
#[doc = "Field `MC` writer - Mode control"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mc, crate::Safe>;
impl<'a, REG> McW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Stop)
    }
    #[doc = "Up mode: Timer counts up to TBxCL0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Up)
    }
    #[doc = "Continuous mode: Timer counts up to the value set by CNTL"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Continuous)
    }
    #[doc = "Up/down mode: Timer counts up to TBxCL0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Updown)
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - Input divider"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            0 => Id::_1,
            1 => Id::_2,
            2 => Id::_4,
            3 => Id::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Id::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Id::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Id::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Id::_8
    }
}
#[doc = "Field `ID` writer - Input divider"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_8)
    }
}
#[doc = "TimerB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbssel {
    #[doc = "0: TBxCLK"]
    Tbclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
    #[doc = "3: INCLK"]
    Inclk = 3,
}
impl From<Tbssel> for u8 {
    #[inline(always)]
    fn from(variant: Tbssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbssel {
    type Ux = u8;
}
impl crate::IsEnum for Tbssel {}
#[doc = "Field `TBSSEL` reader - TimerB clock source select"]
pub type TbsselR = crate::FieldReader<Tbssel>;
impl TbsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbssel {
        match self.bits {
            0 => Tbssel::Tbclk,
            1 => Tbssel::Aclk,
            2 => Tbssel::Smclk,
            3 => Tbssel::Inclk,
            _ => unreachable!(),
        }
    }
    #[doc = "TBxCLK"]
    #[inline(always)]
    pub fn is_tbclk(&self) -> bool {
        *self == Tbssel::Tbclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Tbssel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Tbssel::Smclk
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == Tbssel::Inclk
    }
}
#[doc = "Field `TBSSEL` writer - TimerB clock source select"]
pub type TbsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbssel, crate::Safe>;
impl<'a, REG> TbsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TBxCLK"]
    #[inline(always)]
    pub fn tbclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Tbclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Smclk)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Inclk)
    }
}
#[doc = "Counter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntl {
    #[doc = "0: 16-bit, TBxR(max) = 0FFFFh"]
    _16 = 0,
    #[doc = "1: 12-bit, TBxR(max) = 0FFFh"]
    _12 = 1,
    #[doc = "2: 10-bit, TBxR(max) = 03FFh"]
    _10 = 2,
    #[doc = "3: 8-bit, TBxR(max) = 0FFh"]
    _8 = 3,
}
impl From<Cntl> for u8 {
    #[inline(always)]
    fn from(variant: Cntl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntl {
    type Ux = u8;
}
impl crate::IsEnum for Cntl {}
#[doc = "Field `CNTL` reader - Counter length"]
pub type CntlR = crate::FieldReader<Cntl>;
impl CntlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntl {
        match self.bits {
            0 => Cntl::_16,
            1 => Cntl::_12,
            2 => Cntl::_10,
            3 => Cntl::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit, TBxR(max) = 0FFFFh"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Cntl::_16
    }
    #[doc = "12-bit, TBxR(max) = 0FFFh"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == Cntl::_12
    }
    #[doc = "10-bit, TBxR(max) = 03FFh"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cntl::_10
    }
    #[doc = "8-bit, TBxR(max) = 0FFh"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Cntl::_8
    }
}
#[doc = "Field `CNTL` writer - Counter length"]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntl, crate::Safe>;
impl<'a, REG> CntlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit, TBxR(max) = 0FFFFh"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::_16)
    }
    #[doc = "12-bit, TBxR(max) = 0FFFh"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::_12)
    }
    #[doc = "10-bit, TBxR(max) = 03FFh"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::_10)
    }
    #[doc = "8-bit, TBxR(max) = 0FFh"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::_8)
    }
}
#[doc = "TBxCLn group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbclgrp {
    #[doc = "0: Each TBxCLn latch loads independently"]
    Tbclgrp0 = 0,
    #[doc = "1: TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent"]
    Tbclgrp1 = 1,
    #[doc = "2: TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent"]
    Tbclgrp2 = 2,
    #[doc = "3: TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)"]
    Tbclgrp3 = 3,
}
impl From<Tbclgrp> for u8 {
    #[inline(always)]
    fn from(variant: Tbclgrp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbclgrp {
    type Ux = u8;
}
impl crate::IsEnum for Tbclgrp {}
#[doc = "Field `TBCLGRP` reader - TBxCLn group"]
pub type TbclgrpR = crate::FieldReader<Tbclgrp>;
impl TbclgrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbclgrp {
        match self.bits {
            0 => Tbclgrp::Tbclgrp0,
            1 => Tbclgrp::Tbclgrp1,
            2 => Tbclgrp::Tbclgrp2,
            3 => Tbclgrp::Tbclgrp3,
            _ => unreachable!(),
        }
    }
    #[doc = "Each TBxCLn latch loads independently"]
    #[inline(always)]
    pub fn is_tbclgrp_0(&self) -> bool {
        *self == Tbclgrp::Tbclgrp0
    }
    #[doc = "TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn is_tbclgrp_1(&self) -> bool {
        *self == Tbclgrp::Tbclgrp1
    }
    #[doc = "TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn is_tbclgrp_2(&self) -> bool {
        *self == Tbclgrp::Tbclgrp2
    }
    #[doc = "TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)"]
    #[inline(always)]
    pub fn is_tbclgrp_3(&self) -> bool {
        *self == Tbclgrp::Tbclgrp3
    }
}
#[doc = "Field `TBCLGRP` writer - TBxCLn group"]
pub type TbclgrpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbclgrp, crate::Safe>;
impl<'a, REG> TbclgrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each TBxCLn latch loads independently"]
    #[inline(always)]
    pub fn tbclgrp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp0)
    }
    #[doc = "TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn tbclgrp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp1)
    }
    #[doc = "TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn tbclgrp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp2)
    }
    #[doc = "TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)"]
    #[inline(always)]
    pub fn tbclgrp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp3)
    }
}
impl R {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TbifgR {
        TbifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TbieR {
        TbieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TbclrR {
        TbclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&self) -> TbsselR {
        TbsselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TbclgrpR {
        TbclgrpR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&mut self) -> TbifgW<'_, Tb0ctlSpec> {
        TbifgW::new(self, 0)
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&mut self) -> TbieW<'_, Tb0ctlSpec> {
        TbieW::new(self, 1)
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&mut self) -> TbclrW<'_, Tb0ctlSpec> {
        TbclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Tb0ctlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, Tb0ctlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&mut self) -> TbsselW<'_, Tb0ctlSpec> {
        TbsselW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, Tb0ctlSpec> {
        CntlW::new(self, 11)
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&mut self) -> TbclgrpW<'_, Tb0ctlSpec> {
        TbclgrpW::new(self, 13)
    }
}
#[doc = "Timer_B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0ctlSpec;
impl crate::RegisterSpec for Tb0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0ctl::R`](R) reader structure"]
impl crate::Readable for Tb0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tb0ctl::W`](W) writer structure"]
impl crate::Writable for Tb0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB0CTL to value 0"]
impl crate::Resettable for Tb0ctlSpec {}
