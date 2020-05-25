#include "graphics/ShaderModule.hpp"

#include "graphics/LogicalDevice.hpp"
#include "types/integer.h"

#include <fstream>

using namespace graphics;

ShaderModule::ShaderModule()
	: mMainOpName("main")
{
}

ShaderModule::ShaderModule(ShaderModule &&other)
{
	*this = std::move(other);
}

ShaderModule::~ShaderModule()
{
	this->destroy();
}

ShaderModule& ShaderModule::operator=(ShaderModule&& other)
{
	this->mMainOpName = other.mMainOpName;
	this->mFileName = other.mFileName;
	this->mStage = other.mStage;
	this->mShader.swap(other.mShader);
	other.destroy();
	return *this;
}

ShaderModule& ShaderModule::setStage(vk::ShaderStageFlagBits stage)
{
	this->mStage = stage;
	return *this;
}

ShaderModule& ShaderModule::setSource(std::string fileName)
{
	this->mFileName = fileName;
	return *this;
}

bool ShaderModule::isLoaded() const
{
	// Checks underlying structure for VK_NULL_HANDLE
	return (bool)this->mShader;
}

std::optional<std::vector<char>> ShaderModule::readBinary() const
{
	std::ifstream file(this->mFileName, std::ios::ate | std::ios::binary);
	if (!file.is_open())
	{
		return std::nullopt;
	}

	uSize fileSize = (uSize)file.tellg();
	std::vector<char> buffer(fileSize);
	file.seekg(0);
	file.read(buffer.data(), fileSize);

	file.close();
	return buffer;
}

void ShaderModule::create(LogicalDevice const *pDevice)
{
	assert(!isLoaded()); // assumes the shader is not loaded
	assert(pDevice != nullptr && pDevice->isValid());

	auto binary = this->readBinary();
	if (!binary.has_value())
	{
		//LogRenderer(logging::ECategory::LOGERROR, "Failed to read compiled SPIR-V shader: %s", filePath.c_str());
		return;
	}

	auto info = vk::ShaderModuleCreateInfo()
		.setCodeSize(binary.value().size())
		.setPCode(reinterpret_cast<ui32 const *>(binary.value().data()));
	mShader = pDevice->mDevice->createShaderModuleUnique(info);
}

void ShaderModule::destroy()
{
	if (this->isLoaded())
	{
		this->mShader.reset();
	}
}

vk::PipelineShaderStageCreateInfo ShaderModule::getPipelineInfo() const
{
	assert(isLoaded());
	return vk::PipelineShaderStageCreateInfo()
		.setStage(mStage).setPName(mMainOpName.c_str())
		.setModule(mShader.get()).setPSpecializationInfo(nullptr);
}
