#include "memory/alloc.h"

#include "Engine.hpp"
#include "memory/MemoryManager.h"

using namespace engine;

void* engineAlloc(uSize size)
{
	Engine *pEngine = nullptr;
	if (Engine::GetChecked(pEngine))
	{
		return pEngine->alloc(size);
	}
	return nullptr;
}

ui8 engineDealloc(void** ptr)
{
	Engine *pEngine = nullptr;
	if (Engine::GetChecked(pEngine))
	{
		pEngine->dealloc(ptr);
		return 1;
	}
	return 0;
}
