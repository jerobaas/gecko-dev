use alloc::{string::String, vec::Vec};
use core::ops::Range;

#[cfg(feature = "trace")]
use {alloc::borrow::Cow, std::io::Write as _};

use crate::id;

//TODO: consider a readable Id that doesn't include the backend

type FileName = String;

pub const FILE_NAME: &str = "trace.ron";

#[cfg(feature = "trace")]
pub(crate) fn new_render_bundle_encoder_descriptor<'a>(
    label: crate::Label<'a>,
    context: &'a super::RenderPassContext,
    depth_read_only: bool,
    stencil_read_only: bool,
) -> crate::command::RenderBundleEncoderDescriptor<'a> {
    crate::command::RenderBundleEncoderDescriptor {
        label,
        color_formats: Cow::Borrowed(&context.attachments.colors),
        depth_stencil: context.attachments.depth_stencil.map(|format| {
            wgt::RenderBundleDepthStencil {
                format,
                depth_read_only,
                stencil_read_only,
            }
        }),
        sample_count: context.sample_count,
        multiview: context.multiview,
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action<'a> {
    Init {
        desc: crate::device::DeviceDescriptor<'a>,
        backend: wgt::Backend,
    },
    ConfigureSurface(
        id::SurfaceId,
        wgt::SurfaceConfiguration<Vec<wgt::TextureFormat>>,
    ),
    CreateBuffer(id::BufferId, crate::resource::BufferDescriptor<'a>),
    FreeBuffer(id::BufferId),
    DestroyBuffer(id::BufferId),
    CreateTexture(id::TextureId, crate::resource::TextureDescriptor<'a>),
    FreeTexture(id::TextureId),
    DestroyTexture(id::TextureId),
    CreateTextureView {
        id: id::TextureViewId,
        parent_id: id::TextureId,
        desc: crate::resource::TextureViewDescriptor<'a>,
    },
    DestroyTextureView(id::TextureViewId),
    CreateSampler(id::SamplerId, crate::resource::SamplerDescriptor<'a>),
    DestroySampler(id::SamplerId),
    GetSurfaceTexture {
        id: id::TextureId,
        parent_id: id::SurfaceId,
    },
    Present(id::SurfaceId),
    DiscardSurfaceTexture(id::SurfaceId),
    CreateBindGroupLayout(
        id::BindGroupLayoutId,
        crate::binding_model::BindGroupLayoutDescriptor<'a>,
    ),
    DestroyBindGroupLayout(id::BindGroupLayoutId),
    CreatePipelineLayout(
        id::PipelineLayoutId,
        crate::binding_model::PipelineLayoutDescriptor<'a>,
    ),
    DestroyPipelineLayout(id::PipelineLayoutId),
    CreateBindGroup(
        id::BindGroupId,
        crate::binding_model::BindGroupDescriptor<'a>,
    ),
    DestroyBindGroup(id::BindGroupId),
    CreateShaderModule {
        id: id::ShaderModuleId,
        desc: crate::pipeline::ShaderModuleDescriptor<'a>,
        data: FileName,
    },
    DestroyShaderModule(id::ShaderModuleId),
    CreateComputePipeline {
        id: id::ComputePipelineId,
        desc: crate::pipeline::ComputePipelineDescriptor<'a>,
        #[cfg_attr(feature = "replay", serde(default))]
        implicit_context: Option<super::ImplicitPipelineContext>,
    },
    DestroyComputePipeline(id::ComputePipelineId),
    CreateRenderPipeline {
        id: id::RenderPipelineId,
        desc: crate::pipeline::RenderPipelineDescriptor<'a>,
        #[cfg_attr(feature = "replay", serde(default))]
        implicit_context: Option<super::ImplicitPipelineContext>,
    },
    DestroyRenderPipeline(id::RenderPipelineId),
    CreatePipelineCache {
        id: id::PipelineCacheId,
        desc: crate::pipeline::PipelineCacheDescriptor<'a>,
    },
    DestroyPipelineCache(id::PipelineCacheId),
    CreateRenderBundle {
        id: id::RenderBundleId,
        desc: crate::command::RenderBundleEncoderDescriptor<'a>,
        base: crate::command::BasePass<crate::command::RenderCommand>,
    },
    DestroyRenderBundle(id::RenderBundleId),
    CreateQuerySet {
        id: id::QuerySetId,
        desc: crate::resource::QuerySetDescriptor<'a>,
    },
    DestroyQuerySet(id::QuerySetId),
    WriteBuffer {
        id: id::BufferId,
        data: FileName,
        range: Range<wgt::BufferAddress>,
        queued: bool,
    },
    WriteTexture {
        to: crate::command::TexelCopyTextureInfo,
        data: FileName,
        layout: wgt::TexelCopyBufferLayout,
        size: wgt::Extent3d,
    },
    Submit(crate::SubmissionIndex, Vec<Command>),
    CreateBlas {
        id: id::BlasId,
        desc: crate::resource::BlasDescriptor<'a>,
        sizes: wgt::BlasGeometrySizeDescriptors,
    },
    DestroyBlas(id::BlasId),
    CreateTlas {
        id: id::TlasId,
        desc: crate::resource::TlasDescriptor<'a>,
    },
    DestroyTlas(id::TlasId),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Command {
    CopyBufferToBuffer {
        src: id::BufferId,
        src_offset: wgt::BufferAddress,
        dst: id::BufferId,
        dst_offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    },
    CopyBufferToTexture {
        src: crate::command::TexelCopyBufferInfo,
        dst: crate::command::TexelCopyTextureInfo,
        size: wgt::Extent3d,
    },
    CopyTextureToBuffer {
        src: crate::command::TexelCopyTextureInfo,
        dst: crate::command::TexelCopyBufferInfo,
        size: wgt::Extent3d,
    },
    CopyTextureToTexture {
        src: crate::command::TexelCopyTextureInfo,
        dst: crate::command::TexelCopyTextureInfo,
        size: wgt::Extent3d,
    },
    ClearBuffer {
        dst: id::BufferId,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    },
    ClearTexture {
        dst: id::TextureId,
        subresource_range: wgt::ImageSubresourceRange,
    },
    WriteTimestamp {
        query_set_id: id::QuerySetId,
        query_index: u32,
    },
    ResolveQuerySet {
        query_set_id: id::QuerySetId,
        start_query: u32,
        query_count: u32,
        destination: id::BufferId,
        destination_offset: wgt::BufferAddress,
    },
    PushDebugGroup(String),
    PopDebugGroup,
    InsertDebugMarker(String),
    RunComputePass {
        base: crate::command::BasePass<crate::command::ComputeCommand>,
        timestamp_writes: Option<crate::command::PassTimestampWrites>,
    },
    RunRenderPass {
        base: crate::command::BasePass<crate::command::RenderCommand>,
        target_colors: Vec<Option<crate::command::RenderPassColorAttachment>>,
        target_depth_stencil: Option<crate::command::RenderPassDepthStencilAttachment>,
        timestamp_writes: Option<crate::command::PassTimestampWrites>,
        occlusion_query_set_id: Option<id::QuerySetId>,
    },
    BuildAccelerationStructures {
        blas: Vec<crate::ray_tracing::TraceBlasBuildEntry>,
        tlas: Vec<crate::ray_tracing::TraceTlasPackage>,
    },
}

#[cfg(feature = "trace")]
#[derive(Debug)]
pub struct Trace {
    path: std::path::PathBuf,
    file: std::fs::File,
    config: ron::ser::PrettyConfig,
    binary_id: usize,
}

#[cfg(feature = "trace")]
impl Trace {
    pub fn new(path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        log::info!("Tracing into '{:?}'", path);
        let mut file = std::fs::File::create(path.join(FILE_NAME))?;
        file.write_all(b"[\n")?;
        Ok(Self {
            path,
            file,
            config: ron::ser::PrettyConfig::default(),
            binary_id: 0,
        })
    }

    pub fn make_binary(&mut self, kind: &str, data: &[u8]) -> String {
        self.binary_id += 1;
        let name = std::format!("data{}.{}", self.binary_id, kind);
        let _ = std::fs::write(self.path.join(&name), data);
        name
    }

    pub(crate) fn add(&mut self, action: Action) {
        match ron::ser::to_string_pretty(&action, self.config.clone()) {
            Ok(string) => {
                let _ = writeln!(self.file, "{},", string);
            }
            Err(e) => {
                log::warn!("RON serialization failure: {:?}", e);
            }
        }
    }
}

#[cfg(feature = "trace")]
impl Drop for Trace {
    fn drop(&mut self) {
        let _ = self.file.write_all(b"]");
    }
}
