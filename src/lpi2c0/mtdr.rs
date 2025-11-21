#[doc = "Register `MTDR` writer"]
pub type W = crate::W<MtdrSpec>;
#[doc = "Field `DATA` writer - Transmit Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Command Data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "0: Transmit value in DATA\\[7:0\\]"]
    Transmit = 0,
    #[doc = "1: Receive (DATA\\[7:0\\] + 1) bytes."]
    Receive = 1,
    #[doc = "2: Generate Stop condition on I2C bus."]
    Stop = 2,
    #[doc = "3: Receive and discard (DATA\\[7:0\\] + 1) bytes."]
    ReceiveAndDiscard = 3,
    #[doc = "4: Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]"]
    Start = 4,
    #[doc = "5: Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] expecting a NACK response"]
    StartExpectNack = 5,
    #[doc = "6: Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode"]
    StartHs = 6,
    #[doc = "7: Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode expecting a NACK response"]
    StartHsExpectNack = 7,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` writer - Command Data"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmd, crate::Safe>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit value in DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Transmit)
    }
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes."]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Receive)
    }
    #[doc = "Generate Stop condition on I2C bus."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Stop)
    }
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes."]
    #[inline(always)]
    pub fn receive_and_discard(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::ReceiveAndDiscard)
    }
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Start)
    }
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] expecting a NACK response"]
    #[inline(always)]
    pub fn start_expect_nack(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::StartExpectNack)
    }
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode"]
    #[inline(always)]
    pub fn start_hs(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::StartHs)
    }
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode expecting a NACK response"]
    #[inline(always)]
    pub fn start_hs_expect_nack(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::StartHsExpectNack)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MtdrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, MtdrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, MtdrSpec> {
        CmdW::new(self, 8)
    }
}
#[doc = "Controller Transmit Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtdrSpec;
impl crate::RegisterSpec for MtdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mtdr::W`](W) writer structure"]
impl crate::Writable for MtdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTDR to value 0"]
impl crate::Resettable for MtdrSpec {}
