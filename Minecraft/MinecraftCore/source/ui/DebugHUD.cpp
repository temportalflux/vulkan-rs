#include "ui/DebugHUD.hpp"

#include "game/GameClient.hpp"
#include "game/GameInstance.hpp"
#include "Window.hpp"
#include "ecs/entity/Entity.hpp"
#include "ecs/component/CoordinateTransform.hpp"
#include "ui/UIWidgets.hpp"
#include "ui/WidgetRenderer.hpp"
#include "utility/StringUtils.hpp"

using namespace ui;

DebugHUD::DebugHUD()
{
	(*(this->mpAlphabet = std::make_shared<ui::Text>()))
		.setFontOwner(game::Game::Get()->client()->uiFontOwner())
		.setFont("unispace").setFontSize(30)
		.setContent("Sphinx of Black Quartz, Judge my vow", true);

	(*(this->mpPosition = std::make_shared<ui::Text>()))
		.setFontOwner(game::Game::Get()->client()->uiFontOwner())
		.setFont("unispace").setFontSize(15)
		.setPosition({ 0, 40 })
		.setContent("Position| X:<?,?,?.??> Y:<?,?,?.??> Z:<?,?,?.??>", true);

	(*(this->mpFPS = std::make_shared<ui::Text>()))
		.setFontOwner(game::Game::Get()->client()->uiFontOwner())
		.setFont("unispace").setFontSize(20)
		.setAnchor({ 1, 0 }).setPivot({ 1, 0 })
		.setContent("FPS: ###", true);

	this->setIsVisible(true);
}

DebugHUD::~DebugHUD()
{
	this->mpAlphabet.reset();
	this->mpPosition.reset();
	this->mpFPS.reset();
}

void DebugHUD::addWidgetsToRenderer(ui::WidgetRenderer *renderer)
{
	renderer->add(this->mpAlphabet);
	renderer->add(this->mpPosition);
	renderer->add(this->mpFPS);
}

void DebugHUD::setIsVisible(bool bVisible)
{
	this->mpAlphabet->setIsVisible(bVisible);
	this->mpPosition->setIsVisible(bVisible);
	this->mpFPS->setIsVisible(bVisible);
}

void DebugHUD::tick(f32 deltaTime)
{
	this->mOccurance = (this->mOccurance + 1) % 6000;
	if (this->mOccurance != 0) return;

	auto pGame = game::Game::Get();
	
	/*
	auto const& transform = pGame->localPlayer()->getComponent<ecs::component::CoordinateTransform>();
	auto const& pos = transform->position();
	this->mpPosition->setContent(utility::formatStr(
		"Position| X:<%i,%i,%.2f> Y:<%i,%i,%.2f> Z:<%i,%i,%.2f>",
		pos.chunk().x(), pos.local().x(), pos.offset().x(),
		pos.chunk().y(), pos.local().y(), pos.offset().y(),
		pos.chunk().z(), pos.local().z(), pos.offset().z()
	));
	//*/

	auto deltaMS = pGame->client()->getWindow()->renderDurationMS();
	i32 fps = i32((1.0f / deltaMS) * 1000.0f);
	this->mpFPS->setContent(utility::formatStr("FPS %i", fps));
}
