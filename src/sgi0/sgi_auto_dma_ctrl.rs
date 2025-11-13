#[doc = "Register `sgi_auto_dma_ctrl` reader"]
pub type R = crate::R<SgiAutoDmaCtrlSpec>;
#[doc = "Register `sgi_auto_dma_ctrl` writer"]
pub type W = crate::W<SgiAutoDmaCtrlSpec>;
#[doc = "Field `ife` reader - Input FIFO DMA Enable"]
pub type IfeR = crate::BitReader;
#[doc = "Field `ife` writer - Input FIFO DMA Enable"]
pub type IfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto_dma_rsvd1` reader - reserved"]
pub type AutoDmaRsvd1R = crate::FieldReader;
#[doc = "Field `ofe` reader - Ouput FIFO DMA Enable"]
pub type OfeR = crate::BitReader;
#[doc = "Field `ofe` writer - Ouput FIFO DMA Enable"]
pub type OfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto_dma_rsvd2` reader - reserved"]
pub type AutoDmaRsvd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Input FIFO DMA Enable"]
    #[inline(always)]
    pub fn ife(&self) -> IfeR {
        IfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - reserved"]
    #[inline(always)]
    pub fn auto_dma_rsvd1(&self) -> AutoDmaRsvd1R {
        AutoDmaRsvd1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Ouput FIFO DMA Enable"]
    #[inline(always)]
    pub fn ofe(&self) -> OfeR {
        OfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - reserved"]
    #[inline(always)]
    pub fn auto_dma_rsvd2(&self) -> AutoDmaRsvd2R {
        AutoDmaRsvd2R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sgi_auto_dma_ctrl")
            .field("ife", &self.ife())
            .field("auto_dma_rsvd1", &self.auto_dma_rsvd1())
            .field("ofe", &self.ofe())
            .field("auto_dma_rsvd2", &self.auto_dma_rsvd2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Input FIFO DMA Enable"]
    #[inline(always)]
    pub fn ife(&mut self) -> IfeW<'_, SgiAutoDmaCtrlSpec> {
        IfeW::new(self, 0)
    }
    #[doc = "Bit 8 - Ouput FIFO DMA Enable"]
    #[inline(always)]
    pub fn ofe(&mut self) -> OfeW<'_, SgiAutoDmaCtrlSpec> {
        OfeW::new(self, 8)
    }
}
#[doc = "SGI Auto Mode Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgi_auto_dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgi_auto_dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgiAutoDmaCtrlSpec;
impl crate::RegisterSpec for SgiAutoDmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgi_auto_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for SgiAutoDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sgi_auto_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for SgiAutoDmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sgi_auto_dma_ctrl to value 0"]
impl crate::Resettable for SgiAutoDmaCtrlSpec {}
