#[doc = "Register `PSOR` reader"]
pub type R = crate::R<PsorSpec>;
#[doc = "Register `PSOR` writer"]
pub type W = crate::W<PsorSpec>;
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptso {
    #[doc = "0: No change"]
    Ptso0 = 0,
    #[doc = "1: Corresponding field in PDOR becomes 1"]
    Ptso1 = 1,
}
impl From<Ptso> for bool {
    #[inline(always)]
    fn from(variant: Ptso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO(0-31)` reader - Port Set Output"]
pub type PtsoR = crate::BitReader<Ptso>;
impl PtsoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptso {
        match self.bits {
            false => Ptso::Ptso0,
            true => Ptso::Ptso1,
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_ptso0(&self) -> bool {
        *self == Ptso::Ptso0
    }
    #[doc = "Corresponding field in PDOR becomes 1"]
    #[inline(always)]
    pub fn is_ptso1(&self) -> bool {
        *self == Ptso::Ptso1
    }
}
#[doc = "Field `PTSO(0-31)` writer - Port Set Output"]
pub type PtsoW<'a, REG> = crate::BitWriter<'a, REG, Ptso>;
impl<'a, REG> PtsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change"]
    #[inline(always)]
    pub fn ptso0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::Ptso0)
    }
    #[doc = "Corresponding field in PDOR becomes 1"]
    #[inline(always)]
    pub fn ptso1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::Ptso1)
    }
}
impl R {
    #[doc = "Port Set Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTSO0` field.</div>"]
    #[inline(always)]
    pub fn ptso(&self, n: u8) -> PtsoR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PtsoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub fn ptso_iter(&self) -> impl Iterator<Item = PtsoR> + '_ {
        (0..32).map(move |n| PtsoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Set Output"]
    #[inline(always)]
    pub fn ptso0(&self) -> PtsoR {
        PtsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Set Output"]
    #[inline(always)]
    pub fn ptso1(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Set Output"]
    #[inline(always)]
    pub fn ptso2(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Set Output"]
    #[inline(always)]
    pub fn ptso3(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Set Output"]
    #[inline(always)]
    pub fn ptso4(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Set Output"]
    #[inline(always)]
    pub fn ptso5(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Set Output"]
    #[inline(always)]
    pub fn ptso6(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Set Output"]
    #[inline(always)]
    pub fn ptso7(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Set Output"]
    #[inline(always)]
    pub fn ptso8(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Set Output"]
    #[inline(always)]
    pub fn ptso9(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Set Output"]
    #[inline(always)]
    pub fn ptso10(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Set Output"]
    #[inline(always)]
    pub fn ptso11(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Set Output"]
    #[inline(always)]
    pub fn ptso12(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Set Output"]
    #[inline(always)]
    pub fn ptso13(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Set Output"]
    #[inline(always)]
    pub fn ptso14(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Set Output"]
    #[inline(always)]
    pub fn ptso15(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Set Output"]
    #[inline(always)]
    pub fn ptso16(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Set Output"]
    #[inline(always)]
    pub fn ptso17(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Set Output"]
    #[inline(always)]
    pub fn ptso18(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Set Output"]
    #[inline(always)]
    pub fn ptso19(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Set Output"]
    #[inline(always)]
    pub fn ptso20(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Set Output"]
    #[inline(always)]
    pub fn ptso21(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Set Output"]
    #[inline(always)]
    pub fn ptso22(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Set Output"]
    #[inline(always)]
    pub fn ptso23(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Set Output"]
    #[inline(always)]
    pub fn ptso24(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Set Output"]
    #[inline(always)]
    pub fn ptso25(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Set Output"]
    #[inline(always)]
    pub fn ptso26(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Set Output"]
    #[inline(always)]
    pub fn ptso27(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Set Output"]
    #[inline(always)]
    pub fn ptso28(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Set Output"]
    #[inline(always)]
    pub fn ptso29(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Set Output"]
    #[inline(always)]
    pub fn ptso30(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso31(&self) -> PtsoR {
        PtsoR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSOR")
            .field("ptso0", &self.ptso0())
            .field("ptso1", &self.ptso1())
            .field("ptso2", &self.ptso2())
            .field("ptso3", &self.ptso3())
            .field("ptso4", &self.ptso4())
            .field("ptso5", &self.ptso5())
            .field("ptso6", &self.ptso6())
            .field("ptso7", &self.ptso7())
            .field("ptso8", &self.ptso8())
            .field("ptso9", &self.ptso9())
            .field("ptso10", &self.ptso10())
            .field("ptso11", &self.ptso11())
            .field("ptso12", &self.ptso12())
            .field("ptso13", &self.ptso13())
            .field("ptso14", &self.ptso14())
            .field("ptso15", &self.ptso15())
            .field("ptso16", &self.ptso16())
            .field("ptso17", &self.ptso17())
            .field("ptso18", &self.ptso18())
            .field("ptso19", &self.ptso19())
            .field("ptso20", &self.ptso20())
            .field("ptso21", &self.ptso21())
            .field("ptso22", &self.ptso22())
            .field("ptso23", &self.ptso23())
            .field("ptso24", &self.ptso24())
            .field("ptso25", &self.ptso25())
            .field("ptso26", &self.ptso26())
            .field("ptso27", &self.ptso27())
            .field("ptso28", &self.ptso28())
            .field("ptso29", &self.ptso29())
            .field("ptso30", &self.ptso30())
            .field("ptso31", &self.ptso31())
            .finish()
    }
}
impl W {
    #[doc = "Port Set Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTSO0` field.</div>"]
    #[inline(always)]
    pub fn ptso(&mut self, n: u8) -> PtsoW<'_, PsorSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PtsoW::new(self, n)
    }
    #[doc = "Bit 0 - Port Set Output"]
    #[inline(always)]
    pub fn ptso0(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Set Output"]
    #[inline(always)]
    pub fn ptso1(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Set Output"]
    #[inline(always)]
    pub fn ptso2(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Set Output"]
    #[inline(always)]
    pub fn ptso3(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Set Output"]
    #[inline(always)]
    pub fn ptso4(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Set Output"]
    #[inline(always)]
    pub fn ptso5(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Set Output"]
    #[inline(always)]
    pub fn ptso6(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Set Output"]
    #[inline(always)]
    pub fn ptso7(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Set Output"]
    #[inline(always)]
    pub fn ptso8(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Set Output"]
    #[inline(always)]
    pub fn ptso9(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Set Output"]
    #[inline(always)]
    pub fn ptso10(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Set Output"]
    #[inline(always)]
    pub fn ptso11(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Set Output"]
    #[inline(always)]
    pub fn ptso12(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Set Output"]
    #[inline(always)]
    pub fn ptso13(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Set Output"]
    #[inline(always)]
    pub fn ptso14(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Set Output"]
    #[inline(always)]
    pub fn ptso15(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Set Output"]
    #[inline(always)]
    pub fn ptso16(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Set Output"]
    #[inline(always)]
    pub fn ptso17(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Set Output"]
    #[inline(always)]
    pub fn ptso18(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Set Output"]
    #[inline(always)]
    pub fn ptso19(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Set Output"]
    #[inline(always)]
    pub fn ptso20(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Set Output"]
    #[inline(always)]
    pub fn ptso21(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Set Output"]
    #[inline(always)]
    pub fn ptso22(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Set Output"]
    #[inline(always)]
    pub fn ptso23(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Set Output"]
    #[inline(always)]
    pub fn ptso24(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Set Output"]
    #[inline(always)]
    pub fn ptso25(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Set Output"]
    #[inline(always)]
    pub fn ptso26(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Set Output"]
    #[inline(always)]
    pub fn ptso27(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Set Output"]
    #[inline(always)]
    pub fn ptso28(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Set Output"]
    #[inline(always)]
    pub fn ptso29(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Set Output"]
    #[inline(always)]
    pub fn ptso30(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso31(&mut self) -> PtsoW<'_, PsorSpec> {
        PtsoW::new(self, 31)
    }
}
#[doc = "Port Set Output\n\nYou can [`read`](crate::Reg::read) this register and get [`psor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsorSpec;
impl crate::RegisterSpec for PsorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psor::R`](R) reader structure"]
impl crate::Readable for PsorSpec {}
#[doc = "`write(|w| ..)` method takes [`psor::W`](W) writer structure"]
impl crate::Writable for PsorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PsorSpec {}
