#[doc = "Register `PTOR` reader"]
pub type R = crate::R<PtorSpec>;
#[doc = "Register `PTOR` writer"]
pub type W = crate::W<PtorSpec>;
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptto {
    #[doc = "0: No change"]
    Ptto0 = 0,
    #[doc = "1: Set to the inverse of its current logic state"]
    Ptto1 = 1,
}
impl From<Ptto> for bool {
    #[inline(always)]
    fn from(variant: Ptto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO(0-31)` reader - Port Toggle Output"]
pub type PttoR = crate::BitReader<Ptto>;
impl PttoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptto {
        match self.bits {
            false => Ptto::Ptto0,
            true => Ptto::Ptto1,
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_ptto0(&self) -> bool {
        *self == Ptto::Ptto0
    }
    #[doc = "Set to the inverse of its current logic state"]
    #[inline(always)]
    pub fn is_ptto1(&self) -> bool {
        *self == Ptto::Ptto1
    }
}
#[doc = "Field `PTTO(0-31)` writer - Port Toggle Output"]
pub type PttoW<'a, REG> = crate::BitWriter<'a, REG, Ptto>;
impl<'a, REG> PttoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change"]
    #[inline(always)]
    pub fn ptto0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::Ptto0)
    }
    #[doc = "Set to the inverse of its current logic state"]
    #[inline(always)]
    pub fn ptto1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::Ptto1)
    }
}
impl R {
    #[doc = "Port Toggle Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTTO0` field.</div>"]
    #[inline(always)]
    pub fn ptto(&self, n: u8) -> PttoR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PttoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub fn ptto_iter(&self) -> impl Iterator<Item = PttoR> + '_ {
        (0..32).map(move |n| PttoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto0(&self) -> PttoR {
        PttoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto1(&self) -> PttoR {
        PttoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto2(&self) -> PttoR {
        PttoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto3(&self) -> PttoR {
        PttoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto4(&self) -> PttoR {
        PttoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto5(&self) -> PttoR {
        PttoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto6(&self) -> PttoR {
        PttoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto7(&self) -> PttoR {
        PttoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto8(&self) -> PttoR {
        PttoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto9(&self) -> PttoR {
        PttoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto10(&self) -> PttoR {
        PttoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto11(&self) -> PttoR {
        PttoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto12(&self) -> PttoR {
        PttoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto13(&self) -> PttoR {
        PttoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto14(&self) -> PttoR {
        PttoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto15(&self) -> PttoR {
        PttoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto16(&self) -> PttoR {
        PttoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto17(&self) -> PttoR {
        PttoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto18(&self) -> PttoR {
        PttoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto19(&self) -> PttoR {
        PttoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto20(&self) -> PttoR {
        PttoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto21(&self) -> PttoR {
        PttoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto22(&self) -> PttoR {
        PttoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto23(&self) -> PttoR {
        PttoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto24(&self) -> PttoR {
        PttoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto25(&self) -> PttoR {
        PttoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto26(&self) -> PttoR {
        PttoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto27(&self) -> PttoR {
        PttoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto28(&self) -> PttoR {
        PttoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto29(&self) -> PttoR {
        PttoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto30(&self) -> PttoR {
        PttoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto31(&self) -> PttoR {
        PttoR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTOR")
            .field("ptto0", &self.ptto0())
            .field("ptto1", &self.ptto1())
            .field("ptto2", &self.ptto2())
            .field("ptto3", &self.ptto3())
            .field("ptto4", &self.ptto4())
            .field("ptto5", &self.ptto5())
            .field("ptto6", &self.ptto6())
            .field("ptto7", &self.ptto7())
            .field("ptto8", &self.ptto8())
            .field("ptto9", &self.ptto9())
            .field("ptto10", &self.ptto10())
            .field("ptto11", &self.ptto11())
            .field("ptto12", &self.ptto12())
            .field("ptto13", &self.ptto13())
            .field("ptto14", &self.ptto14())
            .field("ptto15", &self.ptto15())
            .field("ptto16", &self.ptto16())
            .field("ptto17", &self.ptto17())
            .field("ptto18", &self.ptto18())
            .field("ptto19", &self.ptto19())
            .field("ptto20", &self.ptto20())
            .field("ptto21", &self.ptto21())
            .field("ptto22", &self.ptto22())
            .field("ptto23", &self.ptto23())
            .field("ptto24", &self.ptto24())
            .field("ptto25", &self.ptto25())
            .field("ptto26", &self.ptto26())
            .field("ptto27", &self.ptto27())
            .field("ptto28", &self.ptto28())
            .field("ptto29", &self.ptto29())
            .field("ptto30", &self.ptto30())
            .field("ptto31", &self.ptto31())
            .finish()
    }
}
impl W {
    #[doc = "Port Toggle Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PTTO0` field.</div>"]
    #[inline(always)]
    pub fn ptto(&mut self, n: u8) -> PttoW<'_, PtorSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PttoW::new(self, n)
    }
    #[doc = "Bit 0 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto0(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto1(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto2(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto3(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto4(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto5(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto6(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto7(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto8(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto9(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto10(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto11(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto12(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto13(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto14(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto15(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto16(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto17(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto18(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto19(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto20(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto21(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto22(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto23(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto24(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto25(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto26(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto27(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto28(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto29(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto30(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto31(&mut self) -> PttoW<'_, PtorSpec> {
        PttoW::new(self, 31)
    }
}
#[doc = "Port Toggle Output\n\nYou can [`read`](crate::Reg::read) this register and get [`ptor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtorSpec;
impl crate::RegisterSpec for PtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptor::R`](R) reader structure"]
impl crate::Readable for PtorSpec {}
#[doc = "`write(|w| ..)` method takes [`ptor::W`](W) writer structure"]
impl crate::Writable for PtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PtorSpec {}
