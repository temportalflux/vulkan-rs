#include "graphics/FontAtlas.hpp"

using namespace graphics;

std::optional<uIndex> Font::findSet(ui8 size) const
{
	auto iter = std::find(this->mSupportedSizes.begin(), this->mSupportedSizes.end(), size);
	return iter != this->mSupportedSizes.end() ? std::make_optional(std::distance(this->mSupportedSizes.begin(), iter)) : std::nullopt;
}

Font::Face& Font::getFace(ui8 size)
{
	auto idxSet = this->findSet(size);
	assert(idxSet);
	return this->mGlyphFaces[*idxSet];
}

std::vector<Font::Face>& Font::faces()
{
	return this->mGlyphFaces;
}

graphics::ImageSampler& Font::Face::sampler()
{
	return this->mSampler;
}

graphics::Image& Font::Face::image()
{
	return this->mImage;
}

graphics::ImageView& Font::Face::view()
{
	return this->mView;
}

math::Vector2UInt Font::Face::getAtlasSize() const
{
	return this->atlasSize;
}

std::vector<ui8>& Font::Face::getPixelData()
{
	return this->textureData;
}

Font& Font::loadGlyphSets(std::vector<ui8> const &fontSizes, std::vector<graphics::FontGlyphSet> const &glyphSets)
{
	assert(fontSizes.size() == glyphSets.size());
	uSize setCount = fontSizes.size();
	this->mSupportedSizes = fontSizes;
	this->mGlyphFaces.resize(setCount);
	for (uIndex idxSet = 0; idxSet < setCount; idxSet++)
	{
		this->mGlyphFaces[idxSet].fontSize = this->mSupportedSizes[idxSet];
		this->mGlyphFaces[idxSet].loadGlyphSet(glyphSets[idxSet]);
	}
	return *this;
}

void Font::Face::loadGlyphSet(FontGlyphSet const &src)
{
	// Copy over glyph metadata
	auto glyphCount = src.glyphs.size();
	this->glyphs.resize(glyphCount);
	this->codeToGlyphIdx = src.codeToGlyphIdx;
	for (auto& [charCode, glyphIdx] : src.codeToGlyphIdx)
	{
		this->glyphs[glyphIdx] = src.glyphs[glyphIdx];
	}
	
	// Determine the atlas size required for the glyphs
	this->atlasSize = this->calculateAtlasLayout();

	// Create the atlas texture
	this->textureData.resize(this->atlasSize.x() * this->atlasSize.y() * 4); // 4 channels RGBA

	// Write glyph buffer data to the face's atlas texture
	for (auto&[charCode, glyphIdx] : src.codeToGlyphIdx)
	{
		const auto& alphaBuffer = src.glyphs[glyphIdx].buffer;
		if (alphaBuffer.size() > 0)
		{
			this->writeAlphaToTexture(
				this->glyphs[glyphIdx].atlasOffset,
				this->glyphs[glyphIdx].bufferSize,
				alphaBuffer
			);
		}
	}
	
}

Font::GlyphSprite& Font::GlyphSprite::operator=(FontGlyph const &other)
{
	this->metricsOffset = other.metricsOffset;
	this->metricsSize = other.metricsSize;
	this->advance = other.advance;
	this->bufferSize = other.bufferSize;
	return *this;
}

// See https://snorristurluson.github.io/TextRenderingWithFreetype/ for reference
math::Vector2UInt Font::Face::measure(std::string str) const
{
	math::Vector2UInt size;
	for (auto& c : str)
	{
		auto& glyph = this->glyphs[c];
		size.x() += glyph.advance;
		size.y() = math::max(size.y(), glyph.atlasOffset.y() + glyph.bufferSize.y());
	}
	return size;
}

math::Vector2UInt Font::Face::calculateAtlasLayout()
{
	// Its very unlikely that the atlas could fit all the glyphs in a size smaller than 256x256
	math::Vector2UInt atlasSize = { 256, 256 };

	bool bCanFitAllGlyphs;
	do
	{
		bCanFitAllGlyphs = true;

		math::Vector2UInt rowSize, rowPos;
		for (auto& glyph : this->glyphs)
		{
			if (!(glyph.bufferSize.x() > 0 && glyph.bufferSize.y() > 0)) continue;
			// Row will be exceeded if the glyph is appended to the current row.
			if (rowSize.x() + glyph.bufferSize.x() > atlasSize.x())
			{
				// Atlas height will be exceeded if the row is shifted, atlas needs to be bigger
				if (rowPos.y() + rowSize.y() > atlasSize.y())
				{
					bCanFitAllGlyphs = false;
					// Bumps atlas size to the next power of 2
					atlasSize = { atlasSize.x() << 1, atlasSize.y() << 1 };
					break;
				}
				// Shift the next row down by the largest size recorded
				rowPos.y() += rowSize.y();
				// Reset the size of the row
				rowSize.x(0).y(0);
			}
			glyph.atlasOffset = { rowSize.x(), rowPos.y() };
			rowSize.x() += glyph.bufferSize.x();
			rowSize.y() = math::max(rowSize.y(), glyph.bufferSize.y());
		}
	}
	while (!bCanFitAllGlyphs);

	return atlasSize;
}

void Font::Face::writeAlphaToTexture(math::Vector2UInt const &pos, math::Vector2UInt const &dimensions, std::vector<ui8> const &alpha)
{
	ui32 const channelCountPerPixel = 4; // 4 channels for rgb+a
	for (ui32 x = 0; x < dimensions.x(); ++x) for (ui32 y = 0; y < dimensions.y(); ++y)
	{
		uIndex idxAlpha = y * dimensions.x() + x;
		math::Vector2UInt pixelPos = pos + math::Vector2UInt({ x, y });
		uIndex const idxPixel = pixelPos.y() * this->atlasSize.x() + pixelPos.x();
		uIndex const idxData = idxPixel * channelCountPerPixel;
		this->textureData[idxData + 0] = 0xff;
		this->textureData[idxData + 1] = 0xff;
		this->textureData[idxData + 2] = 0xff;
		this->textureData[idxData + 3] = alpha[idxAlpha];
	}
}

void Font::invalidate()
{
	for (auto& face : this->mGlyphFaces)
	{
		face.invalidate();
	}
	this->mGlyphFaces.clear();
}

void Font::Face::invalidate()
{
	this->mView.invalidate();
	this->mImage.invalidate();
	this->mSampler.invalidate();
}
