#[doc = "Register `PCOR` reader"]
pub type R = crate::R<PcorSpec>;
#[doc = "Register `PCOR` writer"]
pub type W = crate::W<PcorSpec>;
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptco {
    #[doc = "0: No change"]
    Ptco0 = 0,
    #[doc = "1: Corresponding field in PDOR becomes 0"]
    Ptco1 = 1,
}
impl From<Ptco> for bool {
    #[inline(always)]
    fn from(variant: Ptco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO(0-31)` reader - Port Clear Output"]
pub type PtcoR = crate::BitReader<Ptco>;
impl PtcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptco {
        match self.bits {
            false => Ptco::Ptco0,
            true => Ptco::Ptco1,
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_ptco0(&self) -> bool {
        *self == Ptco::Ptco0
    }
    #[doc = "Corresponding field in PDOR becomes 0"]
    #[inline(always)]
    pub fn is_ptco1(&self) -> bool {
        *self == Ptco::Ptco1
    }
}
#[doc = "Field `PTCO(0-31)` writer - Port Clear Output"]
pub type PtcoW<'a, REG> = crate::BitWriter<'a, REG, Ptco>;
impl<'a, REG> PtcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change"]
    #[inline(always)]
    pub fn ptco0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptco::Ptco0)
    }
    #[doc = "Corresponding field in PDOR becomes 0"]
    #[inline(always)]
    pub fn ptco1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptco::Ptco1)
    }
}
impl R {
    #[doc = "Port Clear Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTCO0` field.</div>"]
    #[inline(always)]
    pub fn ptco(&self, n: u8) -> PtcoR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PtcoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub fn ptco_iter(&self) -> impl Iterator<Item = PtcoR> + '_ {
        (0..32).map(move |n| PtcoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco0(&self) -> PtcoR {
        PtcoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco1(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco2(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco3(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco4(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco5(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco6(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco7(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco8(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco9(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco10(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco11(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco12(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco13(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco14(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco15(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco16(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco17(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco18(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco19(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco20(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco21(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco22(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco23(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco24(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco25(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco26(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco27(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco28(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco29(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco30(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco31(&self) -> PtcoR {
        PtcoR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCOR")
            .field("ptco0", &self.ptco0())
            .field("ptco1", &self.ptco1())
            .field("ptco2", &self.ptco2())
            .field("ptco3", &self.ptco3())
            .field("ptco4", &self.ptco4())
            .field("ptco5", &self.ptco5())
            .field("ptco6", &self.ptco6())
            .field("ptco7", &self.ptco7())
            .field("ptco8", &self.ptco8())
            .field("ptco9", &self.ptco9())
            .field("ptco10", &self.ptco10())
            .field("ptco11", &self.ptco11())
            .field("ptco12", &self.ptco12())
            .field("ptco13", &self.ptco13())
            .field("ptco14", &self.ptco14())
            .field("ptco15", &self.ptco15())
            .field("ptco16", &self.ptco16())
            .field("ptco17", &self.ptco17())
            .field("ptco18", &self.ptco18())
            .field("ptco19", &self.ptco19())
            .field("ptco20", &self.ptco20())
            .field("ptco21", &self.ptco21())
            .field("ptco22", &self.ptco22())
            .field("ptco23", &self.ptco23())
            .field("ptco24", &self.ptco24())
            .field("ptco25", &self.ptco25())
            .field("ptco26", &self.ptco26())
            .field("ptco27", &self.ptco27())
            .field("ptco28", &self.ptco28())
            .field("ptco29", &self.ptco29())
            .field("ptco30", &self.ptco30())
            .field("ptco31", &self.ptco31())
            .finish()
    }
}
impl W {
    #[doc = "Port Clear Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTCO0` field.</div>"]
    #[inline(always)]
    pub fn ptco(&mut self, n: u8) -> PtcoW<'_, PcorSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PtcoW::new(self, n)
    }
    #[doc = "Bit 0 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco0(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco1(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco2(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco3(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco4(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco5(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco6(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco7(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco8(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco9(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco10(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco11(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco12(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco13(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco14(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco15(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco16(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco17(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco18(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco19(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco20(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco21(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco22(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco23(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco24(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco25(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco26(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco27(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco28(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco29(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco30(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco31(&mut self) -> PtcoW<'_, PcorSpec> {
        PtcoW::new(self, 31)
    }
}
#[doc = "Port Clear Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pcor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcorSpec;
impl crate::RegisterSpec for PcorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcor::R`](R) reader structure"]
impl crate::Readable for PcorSpec {}
#[doc = "`write(|w| ..)` method takes [`pcor::W`](W) writer structure"]
impl crate::Writable for PcorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCOR to value 0"]
impl crate::Resettable for PcorSpec {}
