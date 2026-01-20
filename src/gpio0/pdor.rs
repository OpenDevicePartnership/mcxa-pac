#[doc = "Register `PDOR` reader"]
pub type R = crate::R<PdorSpec>;
#[doc = "Register `PDOR` writer"]
pub type W = crate::W<PdorSpec>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdo {
    #[doc = "0: Logic level 0"]
    Pdo0 = 0,
    #[doc = "1: Logic level 1"]
    Pdo1 = 1,
}
impl From<Pdo> for bool {
    #[inline(always)]
    fn from(variant: Pdo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDO(0-31)` reader - Port Data Output"]
pub type PdoR = crate::BitReader<Pdo>;
impl PdoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdo {
        match self.bits {
            false => Pdo::Pdo0,
            true => Pdo::Pdo1,
        }
    }
    #[doc = "Logic level 0"]
    #[inline(always)]
    pub fn is_pdo0(&self) -> bool {
        *self == Pdo::Pdo0
    }
    #[doc = "Logic level 1"]
    #[inline(always)]
    pub fn is_pdo1(&self) -> bool {
        *self == Pdo::Pdo1
    }
}
#[doc = "Field `PDO(0-31)` writer - Port Data Output"]
pub type PdoW<'a, REG> = crate::BitWriter<'a, REG, Pdo>;
impl<'a, REG> PdoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logic level 0"]
    #[inline(always)]
    pub fn pdo0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdo::Pdo0)
    }
    #[doc = "Logic level 1"]
    #[inline(always)]
    pub fn pdo1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdo::Pdo1)
    }
}
impl R {
    #[doc = "Port Data Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PDO0` field.</div>"]
    #[inline(always)]
    pub fn pdo(&self, n: u8) -> PdoR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PdoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub fn pdo_iter(&self) -> impl Iterator<Item = PdoR> + '_ {
        (0..32).map(move |n| PdoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    pub fn pdo0(&self) -> PdoR {
        PdoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    pub fn pdo1(&self) -> PdoR {
        PdoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    pub fn pdo2(&self) -> PdoR {
        PdoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    pub fn pdo3(&self) -> PdoR {
        PdoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    pub fn pdo4(&self) -> PdoR {
        PdoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    pub fn pdo5(&self) -> PdoR {
        PdoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    pub fn pdo6(&self) -> PdoR {
        PdoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    pub fn pdo7(&self) -> PdoR {
        PdoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    pub fn pdo8(&self) -> PdoR {
        PdoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    pub fn pdo9(&self) -> PdoR {
        PdoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    pub fn pdo10(&self) -> PdoR {
        PdoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    pub fn pdo11(&self) -> PdoR {
        PdoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    pub fn pdo12(&self) -> PdoR {
        PdoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    pub fn pdo13(&self) -> PdoR {
        PdoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    pub fn pdo14(&self) -> PdoR {
        PdoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    pub fn pdo15(&self) -> PdoR {
        PdoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    pub fn pdo16(&self) -> PdoR {
        PdoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    pub fn pdo17(&self) -> PdoR {
        PdoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    pub fn pdo18(&self) -> PdoR {
        PdoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    pub fn pdo19(&self) -> PdoR {
        PdoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    pub fn pdo20(&self) -> PdoR {
        PdoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    pub fn pdo21(&self) -> PdoR {
        PdoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    pub fn pdo22(&self) -> PdoR {
        PdoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    pub fn pdo23(&self) -> PdoR {
        PdoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    pub fn pdo24(&self) -> PdoR {
        PdoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    pub fn pdo25(&self) -> PdoR {
        PdoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    pub fn pdo26(&self) -> PdoR {
        PdoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    pub fn pdo27(&self) -> PdoR {
        PdoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    pub fn pdo28(&self) -> PdoR {
        PdoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    pub fn pdo29(&self) -> PdoR {
        PdoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    pub fn pdo30(&self) -> PdoR {
        PdoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo31(&self) -> PdoR {
        PdoR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDOR")
            .field("pdo0", &self.pdo0())
            .field("pdo1", &self.pdo1())
            .field("pdo2", &self.pdo2())
            .field("pdo3", &self.pdo3())
            .field("pdo4", &self.pdo4())
            .field("pdo5", &self.pdo5())
            .field("pdo6", &self.pdo6())
            .field("pdo7", &self.pdo7())
            .field("pdo8", &self.pdo8())
            .field("pdo9", &self.pdo9())
            .field("pdo10", &self.pdo10())
            .field("pdo11", &self.pdo11())
            .field("pdo12", &self.pdo12())
            .field("pdo13", &self.pdo13())
            .field("pdo14", &self.pdo14())
            .field("pdo15", &self.pdo15())
            .field("pdo16", &self.pdo16())
            .field("pdo17", &self.pdo17())
            .field("pdo18", &self.pdo18())
            .field("pdo19", &self.pdo19())
            .field("pdo20", &self.pdo20())
            .field("pdo21", &self.pdo21())
            .field("pdo22", &self.pdo22())
            .field("pdo23", &self.pdo23())
            .field("pdo24", &self.pdo24())
            .field("pdo25", &self.pdo25())
            .field("pdo26", &self.pdo26())
            .field("pdo27", &self.pdo27())
            .field("pdo28", &self.pdo28())
            .field("pdo29", &self.pdo29())
            .field("pdo30", &self.pdo30())
            .field("pdo31", &self.pdo31())
            .finish()
    }
}
impl W {
    #[doc = "Port Data Output"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PDO0` field.</div>"]
    #[inline(always)]
    pub fn pdo(&mut self, n: u8) -> PdoW<'_, PdorSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PdoW::new(self, n)
    }
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    pub fn pdo0(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 0)
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    pub fn pdo1(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    pub fn pdo2(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    pub fn pdo3(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    pub fn pdo4(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 4)
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    pub fn pdo5(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    pub fn pdo6(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    pub fn pdo7(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    pub fn pdo8(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 8)
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    pub fn pdo9(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 9)
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    pub fn pdo10(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 10)
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    pub fn pdo11(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 11)
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    pub fn pdo12(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    pub fn pdo13(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 13)
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    pub fn pdo14(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    pub fn pdo15(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 15)
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    pub fn pdo16(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 16)
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    pub fn pdo17(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 17)
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    pub fn pdo18(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 18)
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    pub fn pdo19(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 19)
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    pub fn pdo20(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 20)
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    pub fn pdo21(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 21)
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    pub fn pdo22(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 22)
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    pub fn pdo23(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 23)
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    pub fn pdo24(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 24)
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    pub fn pdo25(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 25)
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    pub fn pdo26(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 26)
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    pub fn pdo27(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 27)
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    pub fn pdo28(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 28)
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    pub fn pdo29(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 29)
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    pub fn pdo30(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 30)
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo31(&mut self) -> PdoW<'_, PdorSpec> {
        PdoW::new(self, 31)
    }
}
#[doc = "Port Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdorSpec;
impl crate::RegisterSpec for PdorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdor::R`](R) reader structure"]
impl crate::Readable for PdorSpec {}
#[doc = "`write(|w| ..)` method takes [`pdor::W`](W) writer structure"]
impl crate::Writable for PdorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDOR to value 0"]
impl crate::Resettable for PdorSpec {}
