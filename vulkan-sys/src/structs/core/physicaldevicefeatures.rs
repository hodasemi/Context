use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderf3264: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

macro_rules! check_flag {
    ($me: ident, $other: ident, $flag_name: ident) => {
        if $me.$flag_name == VK_TRUE && $other.$flag_name == VK_FALSE {
            return false;
        }
    };
}

impl VkPhysicalDeviceFeatures {
    pub fn is_subset_of(&self, other: &Self) -> bool {
        check_flag!(self, other, robustBufferAccess);
        check_flag!(self, other, fullDrawIndexUint32);
        check_flag!(self, other, imageCubeArray);
        check_flag!(self, other, independentBlend);
        check_flag!(self, other, geometryShader);
        check_flag!(self, other, tessellationShader);
        check_flag!(self, other, sampleRateShading);
        check_flag!(self, other, dualSrcBlend);
        check_flag!(self, other, logicOp);
        check_flag!(self, other, multiDrawIndirect);
        check_flag!(self, other, drawIndirectFirstInstance);
        check_flag!(self, other, depthClamp);
        check_flag!(self, other, depthBiasClamp);
        check_flag!(self, other, fillModeNonSolid);
        check_flag!(self, other, depthBounds);
        check_flag!(self, other, wideLines);
        check_flag!(self, other, largePoints);
        check_flag!(self, other, alphaToOne);
        check_flag!(self, other, multiViewport);
        check_flag!(self, other, samplerAnisotropy);
        check_flag!(self, other, textureCompressionETC2);
        check_flag!(self, other, textureCompressionASTC_LDR);
        check_flag!(self, other, textureCompressionBC);
        check_flag!(self, other, occlusionQueryPrecise);
        check_flag!(self, other, pipelineStatisticsQuery);
        check_flag!(self, other, vertexPipelineStoresAndAtomics);
        check_flag!(self, other, fragmentStoresAndAtomics);
        check_flag!(self, other, shaderTessellationAndGeometryPointSize);
        check_flag!(self, other, shaderImageGatherExtended);
        check_flag!(self, other, shaderStorageImageExtendedFormats);
        check_flag!(self, other, shaderStorageImageMultisample);
        check_flag!(self, other, shaderStorageImageReadWithoutFormat);
        check_flag!(self, other, shaderStorageImageWriteWithoutFormat);
        check_flag!(self, other, shaderUniformBufferArrayDynamicIndexing);
        check_flag!(self, other, shaderSampledImageArrayDynamicIndexing);
        check_flag!(self, other, shaderStorageBufferArrayDynamicIndexing);
        check_flag!(self, other, shaderStorageImageArrayDynamicIndexing);
        check_flag!(self, other, shaderClipDistance);
        check_flag!(self, other, shaderCullDistance);
        check_flag!(self, other, shaderf3264);
        check_flag!(self, other, shaderInt64);
        check_flag!(self, other, shaderInt16);
        check_flag!(self, other, shaderResourceResidency);
        check_flag!(self, other, shaderResourceMinLod);
        check_flag!(self, other, sparseBinding);
        check_flag!(self, other, sparseResidencyBuffer);
        check_flag!(self, other, sparseResidencyImage2D);
        check_flag!(self, other, sparseResidencyImage3D);
        check_flag!(self, other, sparseResidency2Samples);
        check_flag!(self, other, sparseResidency4Samples);
        check_flag!(self, other, sparseResidency8Samples);
        check_flag!(self, other, sparseResidency16Samples);
        check_flag!(self, other, sparseResidencyAliased);
        check_flag!(self, other, variableMultisampleRate);
        check_flag!(self, other, inheritedQueries);

        true
    }
}
