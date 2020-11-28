#pragma once

#include "ecs/component/Component.hpp"

#include "math/Matrix.hpp"
#include "world/WorldCoordinate.hpp"

NS_ECS

class CoordinateTransform : public Component
{
	DECLARE_ECS_COMPONENT_STATICS(16)

public:
	CoordinateTransform();

	world::Coordinate const& position() const;
	CoordinateTransform& setPosition(world::Coordinate const &pos);
	void move(math::Vector3 const &v);

	math::Quaternion const& orientation() const;
	math::Vector3 forward() const;
	math::Vector3 backward() const;
	math::Vector3 right() const;
	math::Vector3 left() const;
	math::Vector3 up() const;
	math::Vector3 down() const;

	CoordinateTransform& setOrientation(math::Quaternion const& orientation);
	CoordinateTransform& setOrientation(math::Vector3 const &axis, f32 const &radians);
	void rotate(math::Vector3 const &axis, f32 const &radians);

	math::Vector3 const& size() const;
	CoordinateTransform& setSize(math::Vector3 const &size);

	/**
	 * Calculates the view matrix based on the current position within the current chunk.
	 */
	math::Matrix4x4 calculateView() const;
	math::Matrix4x4 calculateViewFrom(math::Vector3 const &pos) const;

private:
	world::Coordinate mPosition;
	math::Quaternion mOrientation;
	math::Vector3 mSize;

};

NS_END
