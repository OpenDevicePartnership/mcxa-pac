#[doc = "Register `PDDR` reader"]
pub type R = crate::R<PddrSpec>;
#[doc = "Register `PDDR` writer"]
pub type W = crate::W<PddrSpec>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdd {
    #[doc = "0: Input"]
    Pdd0 = 0,
    #[doc = "1: Output"]
    Pdd1 = 1,
}
impl From<Pdd> for bool {
    #[inline(always)]
    fn from(variant: Pdd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDD(0-31)` reader - Port Data Direction"]
pub type PddR = crate::BitReader<Pdd>;
impl PddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdd {
        match self.bits {
            false => Pdd::Pdd0,
            true => Pdd::Pdd1,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_pdd0(&self) -> bool {
        *self == Pdd::Pdd0
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_pdd1(&self) -> bool {
        *self == Pdd::Pdd1
    }
}
#[doc = "Field `PDD(0-31)` writer - Port Data Direction"]
pub type PddW<'a, REG> = crate::BitWriter<'a, REG, Pdd>;
impl<'a, REG> PddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn pdd0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::Pdd0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn pdd1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::Pdd1)
    }
}
impl R {
    #[doc = "Port Data Direction"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PDD0` field.</div>"]
    #[inline(always)]
    pub fn pdd(&self, n: u8) -> PddR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PddR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub fn pdd_iter(&self) -> impl Iterator<Item = PddR> + '_ {
        (0..32).map(move |n| PddR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd0(&self) -> PddR {
        PddR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd1(&self) -> PddR {
        PddR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd2(&self) -> PddR {
        PddR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd3(&self) -> PddR {
        PddR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd4(&self) -> PddR {
        PddR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd5(&self) -> PddR {
        PddR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd6(&self) -> PddR {
        PddR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd7(&self) -> PddR {
        PddR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd8(&self) -> PddR {
        PddR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd9(&self) -> PddR {
        PddR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd10(&self) -> PddR {
        PddR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd11(&self) -> PddR {
        PddR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd12(&self) -> PddR {
        PddR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd13(&self) -> PddR {
        PddR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd14(&self) -> PddR {
        PddR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd15(&self) -> PddR {
        PddR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd16(&self) -> PddR {
        PddR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd17(&self) -> PddR {
        PddR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd18(&self) -> PddR {
        PddR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd19(&self) -> PddR {
        PddR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd20(&self) -> PddR {
        PddR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd21(&self) -> PddR {
        PddR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd22(&self) -> PddR {
        PddR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd23(&self) -> PddR {
        PddR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd24(&self) -> PddR {
        PddR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd25(&self) -> PddR {
        PddR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd26(&self) -> PddR {
        PddR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd27(&self) -> PddR {
        PddR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd28(&self) -> PddR {
        PddR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd29(&self) -> PddR {
        PddR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd30(&self) -> PddR {
        PddR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd31(&self) -> PddR {
        PddR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDDR")
            .field("pdd0", &self.pdd0())
            .field("pdd1", &self.pdd1())
            .field("pdd2", &self.pdd2())
            .field("pdd3", &self.pdd3())
            .field("pdd4", &self.pdd4())
            .field("pdd5", &self.pdd5())
            .field("pdd6", &self.pdd6())
            .field("pdd7", &self.pdd7())
            .field("pdd8", &self.pdd8())
            .field("pdd9", &self.pdd9())
            .field("pdd10", &self.pdd10())
            .field("pdd11", &self.pdd11())
            .field("pdd12", &self.pdd12())
            .field("pdd13", &self.pdd13())
            .field("pdd14", &self.pdd14())
            .field("pdd15", &self.pdd15())
            .field("pdd16", &self.pdd16())
            .field("pdd17", &self.pdd17())
            .field("pdd18", &self.pdd18())
            .field("pdd19", &self.pdd19())
            .field("pdd20", &self.pdd20())
            .field("pdd21", &self.pdd21())
            .field("pdd22", &self.pdd22())
            .field("pdd23", &self.pdd23())
            .field("pdd24", &self.pdd24())
            .field("pdd25", &self.pdd25())
            .field("pdd26", &self.pdd26())
            .field("pdd27", &self.pdd27())
            .field("pdd28", &self.pdd28())
            .field("pdd29", &self.pdd29())
            .field("pdd30", &self.pdd30())
            .field("pdd31", &self.pdd31())
            .finish()
    }
}
impl W {
    #[doc = "Port Data Direction"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PDD0` field.</div>"]
    #[inline(always)]
    pub fn pdd(&mut self, n: u8) -> PddW<'_, PddrSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PddW::new(self, n)
    }
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd0(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd1(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd2(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd3(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd4(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd5(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd6(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd7(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd8(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd9(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd10(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd11(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd12(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd13(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd14(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd15(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd16(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd17(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd18(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd19(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd20(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd21(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd22(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd23(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd24(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd25(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd26(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd27(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd28(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd29(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd30(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd31(&mut self) -> PddW<'_, PddrSpec> {
        PddW::new(self, 31)
    }
}
#[doc = "Port Data Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PddrSpec;
impl crate::RegisterSpec for PddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pddr::R`](R) reader structure"]
impl crate::Readable for PddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pddr::W`](W) writer structure"]
impl crate::Writable for PddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDDR to value 0"]
impl crate::Resettable for PddrSpec {}
