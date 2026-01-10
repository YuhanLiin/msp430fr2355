#[doc = "Register `UCB0CTLW1` reader"]
pub type R = crate::R<Ucb0ctlw1Spec>;
#[doc = "Register `UCB0CTLW1` writer"]
pub type W = crate::W<Ucb0ctlw1Spec>;
#[doc = "Deglitch time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucglit {
    #[doc = "0: 50 ns"]
    Ucglit0 = 0,
    #[doc = "1: 25 ns"]
    Ucglit1 = 1,
    #[doc = "2: 12.5 ns"]
    Ucglit2 = 2,
    #[doc = "3: 6.25 ns"]
    Ucglit3 = 3,
}
impl From<Ucglit> for u8 {
    #[inline(always)]
    fn from(variant: Ucglit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucglit {
    type Ux = u8;
}
impl crate::IsEnum for Ucglit {}
#[doc = "Field `UCGLIT` reader - Deglitch time"]
pub type UcglitR = crate::FieldReader<Ucglit>;
impl UcglitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucglit {
        match self.bits {
            0 => Ucglit::Ucglit0,
            1 => Ucglit::Ucglit1,
            2 => Ucglit::Ucglit2,
            3 => Ucglit::Ucglit3,
            _ => unreachable!(),
        }
    }
    #[doc = "50 ns"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == Ucglit::Ucglit0
    }
    #[doc = "25 ns"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == Ucglit::Ucglit1
    }
    #[doc = "12.5 ns"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == Ucglit::Ucglit2
    }
    #[doc = "6.25 ns"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == Ucglit::Ucglit3
    }
}
#[doc = "Field `UCGLIT` writer - Deglitch time"]
pub type UcglitW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucglit, crate::Safe>;
impl<'a, REG> UcglitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50 ns"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit0)
    }
    #[doc = "25 ns"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit1)
    }
    #[doc = "12.5 ns"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit2)
    }
    #[doc = "6.25 ns"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit3)
    }
}
#[doc = "Automatic STOP condition generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucastp {
    #[doc = "0: No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care."]
    Ucastp0 = 0,
    #[doc = "1: UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT"]
    Ucastp1 = 1,
    #[doc = "2: A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold"]
    Ucastp2 = 2,
    #[doc = "3: Reserved"]
    Ucastp3 = 3,
}
impl From<Ucastp> for u8 {
    #[inline(always)]
    fn from(variant: Ucastp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucastp {
    type Ux = u8;
}
impl crate::IsEnum for Ucastp {}
#[doc = "Field `UCASTP` reader - Automatic STOP condition generation"]
pub type UcastpR = crate::FieldReader<Ucastp>;
impl UcastpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucastp {
        match self.bits {
            0 => Ucastp::Ucastp0,
            1 => Ucastp::Ucastp1,
            2 => Ucastp::Ucastp2,
            3 => Ucastp::Ucastp3,
            _ => unreachable!(),
        }
    }
    #[doc = "No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care."]
    #[inline(always)]
    pub fn is_ucastp_0(&self) -> bool {
        *self == Ucastp::Ucastp0
    }
    #[doc = "UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT"]
    #[inline(always)]
    pub fn is_ucastp_1(&self) -> bool {
        *self == Ucastp::Ucastp1
    }
    #[doc = "A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold"]
    #[inline(always)]
    pub fn is_ucastp_2(&self) -> bool {
        *self == Ucastp::Ucastp2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_ucastp_3(&self) -> bool {
        *self == Ucastp::Ucastp3
    }
}
#[doc = "Field `UCASTP` writer - Automatic STOP condition generation"]
pub type UcastpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucastp, crate::Safe>;
impl<'a, REG> UcastpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care."]
    #[inline(always)]
    pub fn ucastp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp0)
    }
    #[doc = "UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT"]
    #[inline(always)]
    pub fn ucastp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp1)
    }
    #[doc = "A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold"]
    #[inline(always)]
    pub fn ucastp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn ucastp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp3)
    }
}
#[doc = "SW or HW ACK control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucswack {
    #[doc = "0: The address acknowledge of the slave is controlled by the eUSCI_B module"]
    Ucswack0 = 0,
    #[doc = "1: The user needs to trigger the sending of the address ACK by issuing UCTXACK"]
    Ucswack1 = 1,
}
impl From<Ucswack> for bool {
    #[inline(always)]
    fn from(variant: Ucswack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSWACK` reader - SW or HW ACK control"]
pub type UcswackR = crate::BitReader<Ucswack>;
impl UcswackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucswack {
        match self.bits {
            false => Ucswack::Ucswack0,
            true => Ucswack::Ucswack1,
        }
    }
    #[doc = "The address acknowledge of the slave is controlled by the eUSCI_B module"]
    #[inline(always)]
    pub fn is_ucswack_0(&self) -> bool {
        *self == Ucswack::Ucswack0
    }
    #[doc = "The user needs to trigger the sending of the address ACK by issuing UCTXACK"]
    #[inline(always)]
    pub fn is_ucswack_1(&self) -> bool {
        *self == Ucswack::Ucswack1
    }
}
#[doc = "Field `UCSWACK` writer - SW or HW ACK control"]
pub type UcswackW<'a, REG> = crate::BitWriter<'a, REG, Ucswack>;
impl<'a, REG> UcswackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address acknowledge of the slave is controlled by the eUSCI_B module"]
    #[inline(always)]
    pub fn ucswack_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswack::Ucswack0)
    }
    #[doc = "The user needs to trigger the sending of the address ACK by issuing UCTXACK"]
    #[inline(always)]
    pub fn ucswack_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswack::Ucswack1)
    }
}
#[doc = "ACK all master bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucstpnack {
    #[doc = "0: Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)"]
    Ucstpnack0 = 0,
    #[doc = "1: All bytes are acknowledged by the eUSCI_B when configured as master receiver"]
    Ucstpnack1 = 1,
}
impl From<Ucstpnack> for bool {
    #[inline(always)]
    fn from(variant: Ucstpnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTPNACK` reader - ACK all master bytes"]
pub type UcstpnackR = crate::BitReader<Ucstpnack>;
impl UcstpnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucstpnack {
        match self.bits {
            false => Ucstpnack::Ucstpnack0,
            true => Ucstpnack::Ucstpnack1,
        }
    }
    #[doc = "Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)"]
    #[inline(always)]
    pub fn is_ucstpnack_0(&self) -> bool {
        *self == Ucstpnack::Ucstpnack0
    }
    #[doc = "All bytes are acknowledged by the eUSCI_B when configured as master receiver"]
    #[inline(always)]
    pub fn is_ucstpnack_1(&self) -> bool {
        *self == Ucstpnack::Ucstpnack1
    }
}
#[doc = "Field `UCSTPNACK` writer - ACK all master bytes"]
pub type UcstpnackW<'a, REG> = crate::BitWriter<'a, REG, Ucstpnack>;
impl<'a, REG> UcstpnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)"]
    #[inline(always)]
    pub fn ucstpnack_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpnack::Ucstpnack0)
    }
    #[doc = "All bytes are acknowledged by the eUSCI_B when configured as master receiver"]
    #[inline(always)]
    pub fn ucstpnack_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpnack::Ucstpnack1)
    }
}
#[doc = "Clock low timeout select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucclto {
    #[doc = "0: Disable clock low timeout counter"]
    Ucclto0 = 0,
    #[doc = "1: 135 000 SYSCLK cycles (approximately 28 ms)"]
    Ucclto1 = 1,
    #[doc = "2: 150 000 SYSCLK cycles (approximately 31 ms)"]
    Ucclto2 = 2,
    #[doc = "3: 165 000 SYSCLK cycles (approximately 34 ms)"]
    Ucclto3 = 3,
}
impl From<Ucclto> for u8 {
    #[inline(always)]
    fn from(variant: Ucclto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucclto {
    type Ux = u8;
}
impl crate::IsEnum for Ucclto {}
#[doc = "Field `UCCLTO` reader - Clock low timeout select"]
pub type UccltoR = crate::FieldReader<Ucclto>;
impl UccltoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucclto {
        match self.bits {
            0 => Ucclto::Ucclto0,
            1 => Ucclto::Ucclto1,
            2 => Ucclto::Ucclto2,
            3 => Ucclto::Ucclto3,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable clock low timeout counter"]
    #[inline(always)]
    pub fn is_ucclto_0(&self) -> bool {
        *self == Ucclto::Ucclto0
    }
    #[doc = "135 000 SYSCLK cycles (approximately 28 ms)"]
    #[inline(always)]
    pub fn is_ucclto_1(&self) -> bool {
        *self == Ucclto::Ucclto1
    }
    #[doc = "150 000 SYSCLK cycles (approximately 31 ms)"]
    #[inline(always)]
    pub fn is_ucclto_2(&self) -> bool {
        *self == Ucclto::Ucclto2
    }
    #[doc = "165 000 SYSCLK cycles (approximately 34 ms)"]
    #[inline(always)]
    pub fn is_ucclto_3(&self) -> bool {
        *self == Ucclto::Ucclto3
    }
}
#[doc = "Field `UCCLTO` writer - Clock low timeout select"]
pub type UccltoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucclto, crate::Safe>;
impl<'a, REG> UccltoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable clock low timeout counter"]
    #[inline(always)]
    pub fn ucclto_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto0)
    }
    #[doc = "135 000 SYSCLK cycles (approximately 28 ms)"]
    #[inline(always)]
    pub fn ucclto_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto1)
    }
    #[doc = "150 000 SYSCLK cycles (approximately 31 ms)"]
    #[inline(always)]
    pub fn ucclto_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto2)
    }
    #[doc = "165 000 SYSCLK cycles (approximately 34 ms)"]
    #[inline(always)]
    pub fn ucclto_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto3)
    }
}
#[doc = "Early UCTXIFG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucetxint {
    #[doc = "0: UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit"]
    Ucetxint0 = 0,
    #[doc = "1: UCTXIFG0 is set for each START condition"]
    Ucetxint1 = 1,
}
impl From<Ucetxint> for bool {
    #[inline(always)]
    fn from(variant: Ucetxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCETXINT` reader - Early UCTXIFG0"]
pub type UcetxintR = crate::BitReader<Ucetxint>;
impl UcetxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucetxint {
        match self.bits {
            false => Ucetxint::Ucetxint0,
            true => Ucetxint::Ucetxint1,
        }
    }
    #[doc = "UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit"]
    #[inline(always)]
    pub fn is_ucetxint_0(&self) -> bool {
        *self == Ucetxint::Ucetxint0
    }
    #[doc = "UCTXIFG0 is set for each START condition"]
    #[inline(always)]
    pub fn is_ucetxint_1(&self) -> bool {
        *self == Ucetxint::Ucetxint1
    }
}
#[doc = "Field `UCETXINT` writer - Early UCTXIFG0"]
pub type UcetxintW<'a, REG> = crate::BitWriter<'a, REG, Ucetxint>;
impl<'a, REG> UcetxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit"]
    #[inline(always)]
    pub fn ucetxint_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucetxint::Ucetxint0)
    }
    #[doc = "UCTXIFG0 is set for each START condition"]
    #[inline(always)]
    pub fn ucetxint_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucetxint::Ucetxint1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UcglitR {
        UcglitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&self) -> UcastpR {
        UcastpR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&self) -> UcswackR {
        UcswackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&self) -> UcstpnackR {
        UcstpnackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&self) -> UccltoR {
        UccltoR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&self) -> UcetxintR {
        UcetxintR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UcglitW<'_, Ucb0ctlw1Spec> {
        UcglitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&mut self) -> UcastpW<'_, Ucb0ctlw1Spec> {
        UcastpW::new(self, 2)
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&mut self) -> UcswackW<'_, Ucb0ctlw1Spec> {
        UcswackW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&mut self) -> UcstpnackW<'_, Ucb0ctlw1Spec> {
        UcstpnackW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&mut self) -> UccltoW<'_, Ucb0ctlw1Spec> {
        UccltoW::new(self, 6)
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&mut self) -> UcetxintW<'_, Ucb0ctlw1Spec> {
        UcetxintW::new(self, 8)
    }
}
#[doc = "eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctlw1Spec;
impl crate::RegisterSpec for Ucb0ctlw1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ctlw1::R`](R) reader structure"]
impl crate::Readable for Ucb0ctlw1Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctlw1::W`](W) writer structure"]
impl crate::Writable for Ucb0ctlw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTLW1 to value 0"]
impl crate::Resettable for Ucb0ctlw1Spec {}
