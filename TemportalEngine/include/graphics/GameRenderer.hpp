#pragma once

#include "graphics/VulkanRenderer.hpp"

#include "graphics/AttributeBinding.hpp"
#include "graphics/DescriptorPool.hpp"
#include "graphics/DescriptorGroup.hpp"
#include "graphics/FontAtlas.hpp"
#include "graphics/Image.hpp"
#include "graphics/ImageSampler.hpp"
#include "graphics/ImageView.hpp"

class IRender;
FORWARD_DEF(NS_ASSET, class Font)
FORWARD_DEF(NS_ASSET, class Texture)
FORWARD_DEF(NS_ASSET, class TextureSampler)

NS_GRAPHICS
class Uniform;
class Image;
class Font;

/**
 * A Vulkan renderer tailored for rendering to a single surface using multiple view buffers.
 */
class GameRenderer : public VulkanRenderer
{

public:
	GameRenderer();

	void initializeDevices() override;

	void addRender(IRender *render);

	void setStaticUniform(std::shared_ptr<Uniform> uniform);
	void initializeBuffer(graphics::Buffer &buffer);

	// TODO: Move this to the buffer object
	template <typename TData>
	void writeBufferData(graphics::Buffer &buffer, uSize offset, std::vector<TData> const &dataSet)
	{
		this->writeToBuffer(&buffer, offset, (void*)dataSet.data(), sizeof(TData) * (uSize)dataSet.size());
	}
	
	void setBindings(std::vector<AttributeBinding> bindings);
	void addShader(std::shared_ptr<ShaderModule> shader);
	void setUIShaderBindings(std::shared_ptr<ShaderModule> shaderVert, std::shared_ptr<ShaderModule> shaderFrag, std::vector<AttributeBinding> bindings);

	// Creates an image sampler from some asset
	// TODO: Take in an asset object
	// Returns the idx of the sampler in `mTextureSamplers`
	uIndex createTextureSampler(std::shared_ptr<asset::TextureSampler> sampler);

	// Creates a `graphics::Image` from a `asset::Texture`.
	// Returns the idx of the image view in `mTextureViews`
	uIndex createTextureAssetImage(std::shared_ptr<asset::Texture> texture, uIndex idxSampler);
	void setFont(std::shared_ptr<asset::Font> font);
	
	template <typename TData>
	void setTextToRender(std::vector<TData> const &verticies, std::vector<ui16> const &indicies)
	{
		this->mVertexBufferUI.setSize((uSize)verticies.size() * sizeof(TData));
		this->initializeBuffer(this->mVertexBufferUI);

		this->mIndexBufferUI.setSize((uSize)indicies.size() * sizeof(ui16));
		//this->mIndexBufferUnitType = vk::IndexType::eUint16;
		//this->mIndexCount = (ui32)model.indicies().size();
		this->initializeBuffer(this->mIndexBufferUI);

		this->writeBufferData(this->mVertexBufferUI, 0, verticies);
		this->writeBufferData(this->mIndexBufferUI, 0, indicies);
	}

	void createRenderChain() override;
	void createRenderPass() override;
	RenderPass* getRenderPass() override;
	void destroyRenderPass() override;

	void invalidate() override;

protected:

	void createFrames(uSize viewCount) override;
	uSize getNumberOfFrames() const override;
	graphics::Frame* getFrameAt(uSize idx) override;
	void destroyFrames() override;

private:

	void writeToBuffer(Buffer* buffer, uSize offset, void* data, uSize size);
	void copyBetweenBuffers(Buffer *src, Buffer *dest, uSize size);
	void copyBufferToImage(Buffer *src, Image *dest);
	void transitionImageToLayout(Image *image, vk::ImageLayout prev, vk::ImageLayout next);

	void destroyRenderChain() override;

	void createUniformBuffers();
	void destroyUniformBuffers();
	void createDepthResources(math::Vector2UInt const &resolution);
	void destroyDepthResources();
	void createDescriptors();
	void createCommandObjects();
	void destroyCommandObjects();
	void recordCommandBufferInstructions();

	void prepareRender(ui32 idxCurrentFrame) override;
	void updateUniformBuffer(ui32 idxImageView);
	void render(graphics::Frame* frame, ui32 idxCurrentImage) override;

private:

	CommandPool mCommandPoolTransient;
	std::vector<IRender*> mpRenders;

	RenderPass mRenderPass;

	// Pool for all descriptors that are used in this renderer
	DescriptorPool mDescriptorPool;

	std::shared_ptr<Uniform> mpUniformStatic; // used for global UBO like projection matrix
	std::vector<Buffer> mUniformStaticBuffersPerFrame;
	
	std::vector<ImageSampler> mTextureSamplers;
	std::vector<Image> mTextureImages;
	std::vector<ImageView> mTextureViews;
	// First value of each pair is the image view idx of `mTextureViews`
	// Second value of each pair is the image sampler idx of `mTextureSamplers`
	std::vector<std::pair<uIndex, uIndex>> mTextureDescriptorPairs;
	Image mDepthImage;
	ImageView mDepthView;

	std::vector<FrameBuffer> mFrameBuffers;

	DescriptorGroup mDescriptorGroup;
	Pipeline mPipeline;

	graphics::Font mFont;
	DescriptorGroup mDescriptorGroupUI;
	Pipeline mPipelineUI;
	Buffer mVertexBufferUI, mIndexBufferUI;

	CommandPool mCommandPool;
	std::vector<CommandBuffer> mCommandBuffers;

	std::vector<Frame> mFrames;

	void initializeTransientCommandPool();

};

NS_END
