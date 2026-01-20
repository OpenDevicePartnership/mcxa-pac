#[doc = "Register `PIDR` reader"]
pub type R = crate::R<PidrSpec>;
#[doc = "Register `PIDR` writer"]
pub type W = crate::W<PidrSpec>;
#[doc = "Port Input Disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pid {
    #[doc = "0: Configured for general-purpose input"]
    Pid0 = 0,
    #[doc = "1: Disabled for general-purpose input"]
    Pid1 = 1,
}
impl From<Pid> for bool {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PID(0-31)` reader - Port Input Disable"]
pub type PidR = crate::BitReader<Pid>;
impl PidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            false => Pid::Pid0,
            true => Pid::Pid1,
        }
    }
    #[doc = "Configured for general-purpose input"]
    #[inline(always)]
    pub fn is_pid0(&self) -> bool {
        *self == Pid::Pid0
    }
    #[doc = "Disabled for general-purpose input"]
    #[inline(always)]
    pub fn is_pid1(&self) -> bool {
        *self == Pid::Pid1
    }
}
#[doc = "Field `PID(0-31)` writer - Port Input Disable"]
pub type PidW<'a, REG> = crate::BitWriter<'a, REG, Pid>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured for general-purpose input"]
    #[inline(always)]
    pub fn pid0(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Pid0)
    }
    #[doc = "Disabled for general-purpose input"]
    #[inline(always)]
    pub fn pid1(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Pid1)
    }
}
impl R {
    #[doc = "Port Input Disable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PID0` field.</div>"]
    #[inline(always)]
    pub fn pid(&self, n: u8) -> PidR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PidR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub fn pid_iter(&self) -> impl Iterator<Item = PidR> + '_ {
        (0..32).map(move |n| PidR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Input Disable"]
    #[inline(always)]
    pub fn pid0(&self) -> PidR {
        PidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Input Disable"]
    #[inline(always)]
    pub fn pid1(&self) -> PidR {
        PidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Input Disable"]
    #[inline(always)]
    pub fn pid2(&self) -> PidR {
        PidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Input Disable"]
    #[inline(always)]
    pub fn pid3(&self) -> PidR {
        PidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Input Disable"]
    #[inline(always)]
    pub fn pid4(&self) -> PidR {
        PidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Input Disable"]
    #[inline(always)]
    pub fn pid5(&self) -> PidR {
        PidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Input Disable"]
    #[inline(always)]
    pub fn pid6(&self) -> PidR {
        PidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Input Disable"]
    #[inline(always)]
    pub fn pid7(&self) -> PidR {
        PidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Input Disable"]
    #[inline(always)]
    pub fn pid8(&self) -> PidR {
        PidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Input Disable"]
    #[inline(always)]
    pub fn pid9(&self) -> PidR {
        PidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Input Disable"]
    #[inline(always)]
    pub fn pid10(&self) -> PidR {
        PidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Input Disable"]
    #[inline(always)]
    pub fn pid11(&self) -> PidR {
        PidR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Input Disable"]
    #[inline(always)]
    pub fn pid12(&self) -> PidR {
        PidR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Input Disable"]
    #[inline(always)]
    pub fn pid13(&self) -> PidR {
        PidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Input Disable"]
    #[inline(always)]
    pub fn pid14(&self) -> PidR {
        PidR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Input Disable"]
    #[inline(always)]
    pub fn pid15(&self) -> PidR {
        PidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Input Disable"]
    #[inline(always)]
    pub fn pid16(&self) -> PidR {
        PidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Input Disable"]
    #[inline(always)]
    pub fn pid17(&self) -> PidR {
        PidR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Input Disable"]
    #[inline(always)]
    pub fn pid18(&self) -> PidR {
        PidR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Input Disable"]
    #[inline(always)]
    pub fn pid19(&self) -> PidR {
        PidR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Input Disable"]
    #[inline(always)]
    pub fn pid20(&self) -> PidR {
        PidR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Input Disable"]
    #[inline(always)]
    pub fn pid21(&self) -> PidR {
        PidR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Input Disable"]
    #[inline(always)]
    pub fn pid22(&self) -> PidR {
        PidR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Input Disable"]
    #[inline(always)]
    pub fn pid23(&self) -> PidR {
        PidR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Input Disable"]
    #[inline(always)]
    pub fn pid24(&self) -> PidR {
        PidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Input Disable"]
    #[inline(always)]
    pub fn pid25(&self) -> PidR {
        PidR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Input Disable"]
    #[inline(always)]
    pub fn pid26(&self) -> PidR {
        PidR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Input Disable"]
    #[inline(always)]
    pub fn pid27(&self) -> PidR {
        PidR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Input Disable"]
    #[inline(always)]
    pub fn pid28(&self) -> PidR {
        PidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Input Disable"]
    #[inline(always)]
    pub fn pid29(&self) -> PidR {
        PidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Input Disable"]
    #[inline(always)]
    pub fn pid30(&self) -> PidR {
        PidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid31(&self) -> PidR {
        PidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR")
            .field("pid0", &self.pid0())
            .field("pid1", &self.pid1())
            .field("pid2", &self.pid2())
            .field("pid3", &self.pid3())
            .field("pid4", &self.pid4())
            .field("pid5", &self.pid5())
            .field("pid6", &self.pid6())
            .field("pid7", &self.pid7())
            .field("pid8", &self.pid8())
            .field("pid9", &self.pid9())
            .field("pid10", &self.pid10())
            .field("pid11", &self.pid11())
            .field("pid12", &self.pid12())
            .field("pid13", &self.pid13())
            .field("pid14", &self.pid14())
            .field("pid15", &self.pid15())
            .field("pid16", &self.pid16())
            .field("pid17", &self.pid17())
            .field("pid18", &self.pid18())
            .field("pid19", &self.pid19())
            .field("pid20", &self.pid20())
            .field("pid21", &self.pid21())
            .field("pid22", &self.pid22())
            .field("pid23", &self.pid23())
            .field("pid24", &self.pid24())
            .field("pid25", &self.pid25())
            .field("pid26", &self.pid26())
            .field("pid27", &self.pid27())
            .field("pid28", &self.pid28())
            .field("pid29", &self.pid29())
            .field("pid30", &self.pid30())
            .field("pid31", &self.pid31())
            .finish()
    }
}
impl W {
    #[doc = "Port Input Disable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PID0` field.</div>"]
    #[inline(always)]
    pub fn pid(&mut self, n: u8) -> PidW<'_, PidrSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PidW::new(self, n)
    }
    #[doc = "Bit 0 - Port Input Disable"]
    #[inline(always)]
    pub fn pid0(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Input Disable"]
    #[inline(always)]
    pub fn pid1(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Input Disable"]
    #[inline(always)]
    pub fn pid2(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Input Disable"]
    #[inline(always)]
    pub fn pid3(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Input Disable"]
    #[inline(always)]
    pub fn pid4(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Input Disable"]
    #[inline(always)]
    pub fn pid5(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Input Disable"]
    #[inline(always)]
    pub fn pid6(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Input Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Input Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Input Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Input Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Input Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Input Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Input Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Input Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Input Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Input Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Input Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Input Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Input Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Input Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Input Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Input Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Input Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Input Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Input Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Input Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Input Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Input Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Input Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Input Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> PidW<'_, PidrSpec> {
        PidW::new(self, 31)
    }
}
#[doc = "Port Input Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidrSpec;
impl crate::RegisterSpec for PidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr::R`](R) reader structure"]
impl crate::Readable for PidrSpec {}
#[doc = "`write(|w| ..)` method takes [`pidr::W`](W) writer structure"]
impl crate::Writable for PidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIDR to value 0"]
impl crate::Resettable for PidrSpec {}
