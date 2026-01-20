#[doc = "Register `PDIR` reader"]
pub type R = crate::R<PdirSpec>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdi {
    #[doc = "0: Logic 0"]
    Pdi0 = 0,
    #[doc = "1: Logic 1"]
    Pdi1 = 1,
}
impl From<Pdi> for bool {
    #[inline(always)]
    fn from(variant: Pdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDI(0-31)` reader - Port Data Input"]
pub type PdiR = crate::BitReader<Pdi>;
impl PdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdi {
        match self.bits {
            false => Pdi::Pdi0,
            true => Pdi::Pdi1,
        }
    }
    #[doc = "Logic 0"]
    #[inline(always)]
    pub fn is_pdi0(&self) -> bool {
        *self == Pdi::Pdi0
    }
    #[doc = "Logic 1"]
    #[inline(always)]
    pub fn is_pdi1(&self) -> bool {
        *self == Pdi::Pdi1
    }
}
impl R {
    #[doc = "Port Data Input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PDI0` field.</div>"]
    #[inline(always)]
    pub fn pdi(&self, n: u8) -> PdiR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PdiR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub fn pdi_iter(&self) -> impl Iterator<Item = PdiR> + '_ {
        (0..32).map(move |n| PdiR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port Data Input"]
    #[inline(always)]
    pub fn pdi0(&self) -> PdiR {
        PdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Input"]
    #[inline(always)]
    pub fn pdi1(&self) -> PdiR {
        PdiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Input"]
    #[inline(always)]
    pub fn pdi2(&self) -> PdiR {
        PdiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Input"]
    #[inline(always)]
    pub fn pdi3(&self) -> PdiR {
        PdiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Input"]
    #[inline(always)]
    pub fn pdi4(&self) -> PdiR {
        PdiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Input"]
    #[inline(always)]
    pub fn pdi5(&self) -> PdiR {
        PdiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Input"]
    #[inline(always)]
    pub fn pdi6(&self) -> PdiR {
        PdiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Input"]
    #[inline(always)]
    pub fn pdi7(&self) -> PdiR {
        PdiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Input"]
    #[inline(always)]
    pub fn pdi8(&self) -> PdiR {
        PdiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Input"]
    #[inline(always)]
    pub fn pdi9(&self) -> PdiR {
        PdiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Input"]
    #[inline(always)]
    pub fn pdi10(&self) -> PdiR {
        PdiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Input"]
    #[inline(always)]
    pub fn pdi11(&self) -> PdiR {
        PdiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Input"]
    #[inline(always)]
    pub fn pdi12(&self) -> PdiR {
        PdiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Input"]
    #[inline(always)]
    pub fn pdi13(&self) -> PdiR {
        PdiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Input"]
    #[inline(always)]
    pub fn pdi14(&self) -> PdiR {
        PdiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Input"]
    #[inline(always)]
    pub fn pdi15(&self) -> PdiR {
        PdiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Input"]
    #[inline(always)]
    pub fn pdi16(&self) -> PdiR {
        PdiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Input"]
    #[inline(always)]
    pub fn pdi17(&self) -> PdiR {
        PdiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Input"]
    #[inline(always)]
    pub fn pdi18(&self) -> PdiR {
        PdiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Input"]
    #[inline(always)]
    pub fn pdi19(&self) -> PdiR {
        PdiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Input"]
    #[inline(always)]
    pub fn pdi20(&self) -> PdiR {
        PdiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Input"]
    #[inline(always)]
    pub fn pdi21(&self) -> PdiR {
        PdiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Input"]
    #[inline(always)]
    pub fn pdi22(&self) -> PdiR {
        PdiR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Input"]
    #[inline(always)]
    pub fn pdi23(&self) -> PdiR {
        PdiR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Input"]
    #[inline(always)]
    pub fn pdi24(&self) -> PdiR {
        PdiR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Input"]
    #[inline(always)]
    pub fn pdi25(&self) -> PdiR {
        PdiR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Input"]
    #[inline(always)]
    pub fn pdi26(&self) -> PdiR {
        PdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Input"]
    #[inline(always)]
    pub fn pdi27(&self) -> PdiR {
        PdiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Input"]
    #[inline(always)]
    pub fn pdi28(&self) -> PdiR {
        PdiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Input"]
    #[inline(always)]
    pub fn pdi29(&self) -> PdiR {
        PdiR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Input"]
    #[inline(always)]
    pub fn pdi30(&self) -> PdiR {
        PdiR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi31(&self) -> PdiR {
        PdiR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDIR")
            .field("pdi0", &self.pdi0())
            .field("pdi1", &self.pdi1())
            .field("pdi2", &self.pdi2())
            .field("pdi3", &self.pdi3())
            .field("pdi4", &self.pdi4())
            .field("pdi5", &self.pdi5())
            .field("pdi6", &self.pdi6())
            .field("pdi7", &self.pdi7())
            .field("pdi8", &self.pdi8())
            .field("pdi9", &self.pdi9())
            .field("pdi10", &self.pdi10())
            .field("pdi11", &self.pdi11())
            .field("pdi12", &self.pdi12())
            .field("pdi13", &self.pdi13())
            .field("pdi14", &self.pdi14())
            .field("pdi15", &self.pdi15())
            .field("pdi16", &self.pdi16())
            .field("pdi17", &self.pdi17())
            .field("pdi18", &self.pdi18())
            .field("pdi19", &self.pdi19())
            .field("pdi20", &self.pdi20())
            .field("pdi21", &self.pdi21())
            .field("pdi22", &self.pdi22())
            .field("pdi23", &self.pdi23())
            .field("pdi24", &self.pdi24())
            .field("pdi25", &self.pdi25())
            .field("pdi26", &self.pdi26())
            .field("pdi27", &self.pdi27())
            .field("pdi28", &self.pdi28())
            .field("pdi29", &self.pdi29())
            .field("pdi30", &self.pdi30())
            .field("pdi31", &self.pdi31())
            .finish()
    }
}
#[doc = "Port Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pdir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdirSpec;
impl crate::RegisterSpec for PdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdir::R`](R) reader structure"]
impl crate::Readable for PdirSpec {}
#[doc = "`reset()` method sets PDIR to value 0"]
impl crate::Resettable for PdirSpec {}
