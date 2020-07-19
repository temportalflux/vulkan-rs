#pragma once

#include "TemportalEnginePCH.hpp"

#include "graphics/VulkanRenderer.hpp"
#include "graphics/Surface.hpp"
#include "graphics/ImGuiFrame.hpp"
#include "graphics/DescriptorPool.hpp"

#include <vulkan/vulkan.hpp>

struct ImDrawList;
struct ImDrawCmd;

NS_GUI
class IGui;
NS_END

NS_GRAPHICS
class VulkanInstance;
class Surface;

class ImGuiRenderer : public VulkanRenderer
{
	static void renderImGui(ImDrawList const* parent_list, ImDrawCmd const* cmd);

public:
	ImGuiRenderer();
	
	void initializeDevices() override;
	void finalizeInitialization() override;
	void invalidate() override;

	void addGui(std::string id, std::shared_ptr<gui::IGui> gui);
	std::shared_ptr<gui::IGui> removeGui(std::string id);

	void onInputEvent(void* evt) override;

	void drawFrame() override;

private:

	RenderPass mRenderPass;

	graphics::DescriptorPool mDescriptorPool;
	std::vector<graphics::ImGuiFrame> mGuiFrames;
	ImGuiFrame* mpCurrentFrame;
	math::Vector2Int mpCurrentBufferOffsets;

	void createDescriptorPoolImgui();
	void submitFonts();

	std::unordered_map<std::string, std::shared_ptr<gui::IGui>> mGuis;
	std::vector<std::string> mGuisToRemove;

	void startGuiFrame();
	void makeGui();
	void endGuiFrame();

	void renderDrawData(ImDrawList const* parent_list, ImDrawCmd const* cmd);

protected:
	void createRenderChain() override;
	void destroyRenderChain() override;
	void createRenderPass() override;
	RenderPass* getRenderPass() override;
	void destroyRenderPass() override;

	void createFrames(uSize viewCount) override;
	uSize getNumberOfFrames() const override;
	graphics::Frame* getFrameAt(uSize idx) override;
	void destroyFrames() override;

	void render(graphics::Frame* frame, ui32 idxCurrentImage) override;

};

NS_END