#pragma once

#include "asset/TypedAssetPath.hpp"
#include "asset/PipelineAsset.hpp"
#include "graphics/Descriptor.hpp"
#include "graphics/FontAtlas.hpp"
#include "graphics/ImageSampler.hpp"
#include "thread/MutexLock.hpp"
#include "ui/Core.hpp"
#include "ui/Resolution.hpp"

FORWARD_DEF(NS_GRAPHICS, class Command);
FORWARD_DEF(NS_GRAPHICS, class CommandPool);
FORWARD_DEF(NS_GRAPHICS, class DescriptorPool);
FORWARD_DEF(NS_GRAPHICS, class GraphicsDevice);
FORWARD_DEF(NS_GRAPHICS, class Pipeline);

NS_UI
class Image;
class Widget;

class FontOwner
{
public:
	virtual graphics::Font const& getFont(std::string const& fontId) const = 0;
};

class WidgetRenderer : public std::enable_shared_from_this<WidgetRenderer>
{

public:
	WidgetRenderer();
	~WidgetRenderer();

	graphics::CommandPool* getTransientPool() { return this->mpTransientPool; }

	WidgetRenderer& setImagePipeline(asset::TypedAssetPath<asset::Pipeline> const& path);
	WidgetRenderer& setTextPipeline(asset::TypedAssetPath<asset::Pipeline> const& path);

	void setDevice(std::weak_ptr<graphics::GraphicsDevice> device);
	void initializeData(graphics::CommandPool *pool, graphics::DescriptorPool *descriptorPool);

	void add(std::weak_ptr<ui::Widget> widget);
	void changeZLayer(std::weak_ptr<ui::Widget> widget, ui32 newZ);

	std::shared_ptr<graphics::Pipeline>& imagePipeline() { return this->mImagePipeline; }
	graphics::DescriptorLayout& imageDescriptorLayout() { return this->mImageDescriptorLayout; }
	graphics::ImageSampler& imageSampler() { return this->mImageSampler; }
	std::shared_ptr<graphics::Pipeline>& textPipeline() { return this->mTextPipeline; }
	graphics::DescriptorLayout& textDescriptorLayout() { return this->mTextDescriptorLayout; }
	graphics::ImageSampler& textSampler() { return this->mTextSampler; }

	void createPipeline(math::Vector2UInt const& resolution);
	void record(graphics::Command *command);

	void setAnyWidgetIsDirty();
	bool hasChanges() const;
	void commitWidgets();

private:
	std::weak_ptr<graphics::GraphicsDevice> mpDevice;
	graphics::CommandPool* mpTransientPool;
	graphics::DescriptorPool* mpDescriptorPool;
	ui::Resolution mResolution;

	std::shared_ptr<graphics::Pipeline> mImagePipeline;
	graphics::DescriptorLayout mImageDescriptorLayout;
	graphics::ImageSampler mImageSampler;

	std::shared_ptr<graphics::Pipeline> mTextPipeline;
	graphics::DescriptorLayout mTextDescriptorLayout;
	graphics::ImageSampler mTextSampler;

	struct Layer
	{
		ui32 z;
		std::vector<std::weak_ptr<ui::Widget>> widgets;
		bool operator<(Layer const& other) const;
	};
	std::vector<Layer> mLayers;

	typedef std::pair<std::vector<Layer>::iterator, std::vector<Layer>::iterator> LayerFindResult;
	LayerFindResult findLayer(ui32 z);
	Layer& getOrMakeLayer(ui32 z);

	void initializeWidgetData(std::shared_ptr<ui::Widget> img);
	void commitWidget(std::shared_ptr<ui::Widget> img);

	thread::MutexLock mMutex;
	bool mbAnyWidgetHasChanges;

};

NS_END