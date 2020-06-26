#pragma once

#include "build/asset/BuildAsset.hpp"

#include "math/Vector.hpp"

NS_BUILD

class BuildTexture : public BuildAsset
{

public:
	static std::shared_ptr<BuildAsset> create(std::shared_ptr<asset::Asset> asset);
	static std::vector<ui8> loadImage(std::filesystem::path const &path, math::Vector2UInt &sizeOut);

	BuildTexture() = default;
	BuildTexture(std::shared_ptr<asset::Asset> asset);

	std::vector<std::string> compile() override;
	void save() override;

private:
	std::vector<ui8> mSourceBinary;
	math::Vector2UInt mSourceSize;

};

NS_END
