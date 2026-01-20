#[doc = "Register `GICLR` reader"]
pub type R = crate::R<GiclrSpec>;
#[doc = "Register `GICLR` writer"]
pub type W = crate::W<GiclrSpec>;
#[doc = "Global Interrupt Write Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Giwe {
    #[doc = "0: Not updated"]
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
#[doc = "Field `GIWE(0-15)` reader - Global Interrupt Write Enable"]
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
    #[doc = "Not updated"]
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
#[doc = "Field `GIWE(0-15)` writer - Global Interrupt Write Enable"]
pub type GiweW<'a, REG> = crate::BitWriter<'a, REG, Giwe>;
impl<'a, REG> GiweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not updated"]
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
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GIWE0` field.</div>"]
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
    pub fn giwe0(&self) -> GiweR {
        GiweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe1(&self) -> GiweR {
        GiweR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe2(&self) -> GiweR {
        GiweR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe3(&self) -> GiweR {
        GiweR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe4(&self) -> GiweR {
        GiweR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe5(&self) -> GiweR {
        GiweR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe6(&self) -> GiweR {
        GiweR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe7(&self) -> GiweR {
        GiweR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe8(&self) -> GiweR {
        GiweR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe9(&self) -> GiweR {
        GiweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe10(&self) -> GiweR {
        GiweR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe11(&self) -> GiweR {
        GiweR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe12(&self) -> GiweR {
        GiweR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe13(&self) -> GiweR {
        GiweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe14(&self) -> GiweR {
        GiweR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe15(&self) -> GiweR {
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
        f.debug_struct("GICLR")
            .field("giwe0", &self.giwe0())
            .field("giwe1", &self.giwe1())
            .field("giwe2", &self.giwe2())
            .field("giwe3", &self.giwe3())
            .field("giwe4", &self.giwe4())
            .field("giwe5", &self.giwe5())
            .field("giwe6", &self.giwe6())
            .field("giwe7", &self.giwe7())
            .field("giwe8", &self.giwe8())
            .field("giwe9", &self.giwe9())
            .field("giwe10", &self.giwe10())
            .field("giwe11", &self.giwe11())
            .field("giwe12", &self.giwe12())
            .field("giwe13", &self.giwe13())
            .field("giwe14", &self.giwe14())
            .field("giwe15", &self.giwe15())
            .field("giwd", &self.giwd())
            .finish()
    }
}
impl W {
    #[doc = "Global Interrupt Write Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GIWE0` field.</div>"]
    #[inline(always)]
    pub fn giwe(&mut self, n: u8) -> GiweW<'_, GiclrSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        GiweW::new(self, n)
    }
    #[doc = "Bit 0 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe0(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 0)
    }
    #[doc = "Bit 1 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe1(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 1)
    }
    #[doc = "Bit 2 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe2(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 2)
    }
    #[doc = "Bit 3 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe3(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 3)
    }
    #[doc = "Bit 4 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe4(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 4)
    }
    #[doc = "Bit 5 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe5(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 5)
    }
    #[doc = "Bit 6 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe6(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe7(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe8(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe9(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 9)
    }
    #[doc = "Bit 10 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe10(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 10)
    }
    #[doc = "Bit 11 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe11(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 11)
    }
    #[doc = "Bit 12 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe12(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 12)
    }
    #[doc = "Bit 13 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe13(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 13)
    }
    #[doc = "Bit 14 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe14(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 14)
    }
    #[doc = "Bit 15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe15(&mut self) -> GiweW<'_, GiclrSpec> {
        GiweW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&mut self) -> GiwdW<'_, GiclrSpec> {
        GiwdW::new(self, 16)
    }
}
#[doc = "Global Interrupt Control Low\n\nYou can [`read`](crate::Reg::read) this register and get [`giclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiclrSpec;
impl crate::RegisterSpec for GiclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giclr::R`](R) reader structure"]
impl crate::Readable for GiclrSpec {}
#[doc = "`write(|w| ..)` method takes [`giclr::W`](W) writer structure"]
impl crate::Writable for GiclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GICLR to value 0"]
impl crate::Resettable for GiclrSpec {}
