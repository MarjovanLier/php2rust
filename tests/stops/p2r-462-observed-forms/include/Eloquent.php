<?php

// A small stand-in for Eloquent with Laravel's ObservedBy attribute, an observer and an
// interface (P2R-460, P2R-462). Only the types matter.

namespace Illuminate\Database\Eloquent\Attributes;

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::IS_REPEATABLE)]
class ObservedBy
{
    /**
     * @param array<int, string>|string $classes
     */
    public function __construct(public array|string $classes)
    {
    }
}

namespace Illuminate\Database\Eloquent;

abstract class Model
{
}

namespace App\Observers;

final class CodeObserver
{
    public function created(): void
    {
    }
}

interface Watcher
{
}
