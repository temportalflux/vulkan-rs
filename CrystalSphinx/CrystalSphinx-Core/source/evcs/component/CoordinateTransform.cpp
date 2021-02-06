#include "evcs/component/CoordinateTransform.hpp"

#include "evcs/Core.hpp"
#include "math/transform.hpp"
#include "network/packet/NetworkPacketECSReplicate.hpp"

using namespace evcs;
using namespace evcs::component;

DEFINE_ECS_COMPONENT_STATICS(CoordinateTransform)

CoordinateTransform::CoordinateTransform()
	: mPosition(world::Coordinate(math::Vector3Int::ZERO, math::Vector3Int::ZERO))
	, mOrientation(math::Quaternion::Identity)
	, mSize({ 1, 1, 1 })
{
}

std::vector<Component::Field> CoordinateTransform::allFields() const
{
	return {
		ECS_FIELD(CoordinateTransform, mPosition),
		ECS_FIELD(CoordinateTransform, mOrientation),
		ECS_FIELD(CoordinateTransform, mSize)
	};
}

world::Coordinate const& CoordinateTransform::position() const { return this->mPosition; }
world::Coordinate& CoordinateTransform::position() { return this->mPosition; }
math::Vector3 CoordinateTransform::localPosition() const { return this->mPosition.local().toFloat() + this->mPosition.offset(); }

CoordinateTransform& CoordinateTransform::setPosition(world::Coordinate const &pos)
{
	if (pos == this->mPosition) { return *this; }
	this->mPosition = pos;
	if (evcs::Core::Get()->shouldReplicate())
	{
		this->replicateUpdate()
			->pushComponentField(ECS_REPL_FIELD(CoordinateTransform, mPosition));
	}
	return *this;
}

math::Quaternion const& CoordinateTransform::orientation() const { return this->mOrientation; }
math::Quaternion& CoordinateTransform::orientation() { return this->mOrientation; }

math::Vector3 CoordinateTransform::forward() const
{
	return this->mOrientation.rotate(math::V3_FORWARD);
}

math::Vector3 CoordinateTransform::backward() const
{
	return -forward();
}

math::Vector3 CoordinateTransform::right() const
{
	return this->mOrientation.rotate(math::V3_RIGHT);
}

math::Vector3 CoordinateTransform::left() const
{
	return -right();
}

math::Vector3 CoordinateTransform::up() const
{
	return this->mOrientation.rotate(math::V3_UP);
}

math::Vector3 CoordinateTransform::down() const
{
	return -up();
}

CoordinateTransform& CoordinateTransform::setOrientation(math::Quaternion const& orientation)
{
	this->mOrientation = orientation;
	if (evcs::Core::Get()->shouldReplicate())
	{
		this->replicateUpdate()
			->pushComponentField(ECS_REPL_FIELD(CoordinateTransform, mOrientation));
	}
	return *this;
}

CoordinateTransform& CoordinateTransform::setOrientation(math::Vector3 const &axis, f32 const &radians)
{
	return this->setOrientation(math::Quaternion::FromAxisAngle(axis, radians));
}

void CoordinateTransform::rotate(math::Vector3 const &axis, f32 const &radians)
{
	this->mOrientation = math::Quaternion::concat(this->mOrientation, math::Quaternion::FromAxisAngle(axis, radians));
}

math::Vector3 const& CoordinateTransform::size() const
{
	return this->mSize;
}

CoordinateTransform& CoordinateTransform::setSize(math::Vector3 const &size)
{
	this->mSize = size;
	return *this;
}

math::Matrix4x4 CoordinateTransform::calculateView() const
{
	return this->calculateViewFrom(this->mPosition.local().toFloat() + this->mPosition.offset());
}

math::Matrix4x4 CoordinateTransform::calculateViewFrom(math::Vector3 const &pos) const
{
	OPTICK_EVENT();
	return math::lookAt(pos, pos + this->forward(), this->up());
}
