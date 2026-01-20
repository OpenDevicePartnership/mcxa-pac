#[doc = "Register `GICHR` reader"]
pub type R = crate::R<GichrSpec>;
#[doc = "Register `GICHR` writer"]
pub type W = crate::W<GichrSpec>;
#[doc = "Global Interrupt Write Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Giwe {
    #[doc = "0: Not updated."]
    Giwe0 = 0,
    #[doc = "1: Updated"]
    Giwe1 = 1,
}
impl From<Giwe> for bool {
    #[inline(always)]
    fn from(variant: Giwe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIWE(16-31)` reader - Global Interrupt Write Enable"]
pub type GiweR = crate::BitReader<Giwe>;
impl GiweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Giwe {
        match self.bits {
            false => Giwe::Giwe0,
            true => Giwe::Giwe1,
        }
    }
    #[doc = "Not updated."]
    #[inline(always)]
    pub fn is_giwe0(&self) -> bool {
        *self == Giwe::Giwe0
    }
    #[doc = "Updated"]
    #[inline(always)]
    pub fn is_giwe1(&self) -> bool {
        *self == Giwe::Giwe1
    }
}
#[doc = "Field `GIWE(16-31)` writer - Global Interrupt Write Enable"]
pub type GiweW<'a, REG> = crate::BitWriter<'a, REG, Giwe>;
impl<'a, REG> GiweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not updated."]
    #[inline(always)]
    pub fn giwe0(self) -> &'a mut crate::W<REG> {
        self.variant(Giwe::Giwe0)
    }
    #[doc = "Updated"]
    #[inline(always)]
    pub fn giwe1(self) -> &'a mut crate::W<REG> {
        self.variant(Giwe::Giwe1)
    }
}
#[doc = "Field `GIWD` reader - Global Interrupt Write Data"]
pub type GiwdR = crate::FieldReader<u16>;
#[doc = "Field `GIWD` writer - Global Interrupt Write Data"]
pub type GiwdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Global Interrupt Write Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GIWE16` field.</div>"]
    #[inline(always)]
    pub fn giwe(&self, n: u8) -> GiweR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        GiweR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe_iter(&self) -> impl Iterator<Item = GiweR> + '_ {
        (0..16).map(move |n| GiweR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe16(&self) -> GiweR {
        GiweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe17(&self) -> GiweR {
        GiweR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe18(&self) -> GiweR {
        GiweR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe19(&self) -> GiweR {
        GiweR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe20(&self) -> GiweR {
        GiweR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe21(&self) -> GiweR {
        GiweR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe22(&self) -> GiweR {
        GiweR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe23(&self) -> GiweR {
        GiweR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe24(&self) -> GiweR {
        GiweR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe25(&self) -> GiweR {
        GiweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe26(&self) -> GiweR {
        GiweR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe27(&self) -> GiweR {
        GiweR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe28(&self) -> GiweR {
        GiweR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe29(&self) -> GiweR {
        GiweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe30(&self) -> GiweR {
        GiweR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe31(&self) -> GiweR {
        GiweR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&self) -> GiwdR {
        GiwdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICHR")
            .field("giwe16", &self.giwe16())
            .field("giwe17", &self.giwe17())
            .field("giwe18", &self.giwe18())
            .field("giwe19", &self.giwe19())
            .field("giwe20", &self.giwe20())
            .field("giwe21", &self.giwe21())
            .field("giwe22", &self.giwe22())
            .field("giwe23", &self.giwe23())
            .field("giwe24", &self.giwe24())
            .field("giwe25", &self.giwe25())
            .field("giwe26", &self.giwe26())
            .field("giwe27", &self.giwe27())
            .field("giwe28", &self.giwe28())
            .field("giwe29", &self.giwe29())
            .field("giwe30", &self.giwe30())
            .field("giwe31", &self.giwe31())
            .field("giwd", &self.giwd())
            .finish()
    }
}
impl W {
    #[doc = "Global Interrupt Write Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GIWE16` field.</div>"]
    #[inline(always)]
    pub fn giwe(&mut self, n: u8) -> GiweW<'_, GichrSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        GiweW::new(self, n)
    }
    #[doc = "Bit 0 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe16(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 0)
    }
    #[doc = "Bit 1 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe17(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 1)
    }
    #[doc = "Bit 2 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe18(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 2)
    }
    #[doc = "Bit 3 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe19(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 3)
    }
    #[doc = "Bit 4 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe20(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 4)
    }
    #[doc = "Bit 5 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe21(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 5)
    }
    #[doc = "Bit 6 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe22(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe23(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe24(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe25(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 9)
    }
    #[doc = "Bit 10 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe26(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 10)
    }
    #[doc = "Bit 11 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe27(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 11)
    }
    #[doc = "Bit 12 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe28(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 12)
    }
    #[doc = "Bit 13 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe29(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 13)
    }
    #[doc = "Bit 14 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe30(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 14)
    }
    #[doc = "Bit 15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe31(&mut self) -> GiweW<'_, GichrSpec> {
        GiweW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&mut self) -> GiwdW<'_, GichrSpec> {
        GiwdW::new(self, 16)
    }
}
#[doc = "Global Interrupt Control High\n\nYou can [`read`](crate::Reg::read) this register and get [`gichr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gichr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GichrSpec;
impl crate::RegisterSpec for GichrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gichr::R`](R) reader structure"]
impl crate::Readable for GichrSpec {}
#[doc = "`write(|w| ..)` method takes [`gichr::W`](W) writer structure"]
impl crate::Writable for GichrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GICHR to value 0"]
impl crate::Resettable for GichrSpec {}
