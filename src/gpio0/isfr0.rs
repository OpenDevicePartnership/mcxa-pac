#[doc = "Register `ISFR0` reader"]
pub type R = crate::R<Isfr0Spec>;
#[doc = "Register `ISFR0` writer"]
pub type W = crate::W<Isfr0Spec>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf {
    #[doc = "0: Not detected"]
    Isf0 = 0,
    #[doc = "1: Detected"]
    Isf1 = 1,
}
impl From<Isf> for bool {
    #[inline(always)]
    fn from(variant: Isf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF(0-31)` reader - Interrupt Status Flag"]
pub type IsfR = crate::BitReader<Isf>;
impl IsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf {
        match self.bits {
            false => Isf::Isf0,
            true => Isf::Isf1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_isf0(&self) -> bool {
        *self == Isf::Isf0
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_isf1(&self) -> bool {
        *self == Isf::Isf1
    }
}
#[doc = "Field `ISF(0-31)` writer - Interrupt Status Flag"]
pub type IsfW<'a, REG> = crate::BitWriter1C<'a, REG, Isf>;
impl<'a, REG> IsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn isf0(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::Isf0)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn isf1(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::Isf1)
    }
}
impl R {
    #[doc = "Interrupt Status Flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>"]
    #[inline(always)]
    pub fn isf(&self, n: u8) -> IsfR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        IsfR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf_iter(&self) -> impl Iterator<Item = IsfR> + '_ {
        (0..32).map(move |n| IsfR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&self) -> IsfR {
        IsfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&self) -> IsfR {
        IsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&self) -> IsfR {
        IsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&self) -> IsfR {
        IsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&self) -> IsfR {
        IsfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&self) -> IsfR {
        IsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&self) -> IsfR {
        IsfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&self) -> IsfR {
        IsfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&self) -> IsfR {
        IsfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&self) -> IsfR {
        IsfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&self) -> IsfR {
        IsfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&self) -> IsfR {
        IsfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&self) -> IsfR {
        IsfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&self) -> IsfR {
        IsfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&self) -> IsfR {
        IsfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&self) -> IsfR {
        IsfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&self) -> IsfR {
        IsfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&self) -> IsfR {
        IsfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&self) -> IsfR {
        IsfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&self) -> IsfR {
        IsfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&self) -> IsfR {
        IsfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&self) -> IsfR {
        IsfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&self) -> IsfR {
        IsfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&self) -> IsfR {
        IsfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&self) -> IsfR {
        IsfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&self) -> IsfR {
        IsfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&self) -> IsfR {
        IsfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&self) -> IsfR {
        IsfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&self) -> IsfR {
        IsfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&self) -> IsfR {
        IsfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&self) -> IsfR {
        IsfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&self) -> IsfR {
        IsfR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISFR0")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .field("isf16", &self.isf16())
            .field("isf17", &self.isf17())
            .field("isf18", &self.isf18())
            .field("isf19", &self.isf19())
            .field("isf20", &self.isf20())
            .field("isf21", &self.isf21())
            .field("isf22", &self.isf22())
            .field("isf23", &self.isf23())
            .field("isf24", &self.isf24())
            .field("isf25", &self.isf25())
            .field("isf26", &self.isf26())
            .field("isf27", &self.isf27())
            .field("isf28", &self.isf28())
            .field("isf29", &self.isf29())
            .field("isf30", &self.isf30())
            .field("isf31", &self.isf31())
            .finish()
    }
}
impl W {
    #[doc = "Interrupt Status Flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>"]
    #[inline(always)]
    pub fn isf(&mut self, n: u8) -> IsfW<'_, Isfr0Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        IsfW::new(self, n)
    }
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&mut self) -> IsfW<'_, Isfr0Spec> {
        IsfW::new(self, 31)
    }
}
#[doc = "Interrupt Status Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`isfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isfr0Spec;
impl crate::RegisterSpec for Isfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isfr0::R`](R) reader structure"]
impl crate::Readable for Isfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`isfr0::W`](W) writer structure"]
impl crate::Writable for Isfr0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets ISFR0 to value 0"]
impl crate::Resettable for Isfr0Spec {}
