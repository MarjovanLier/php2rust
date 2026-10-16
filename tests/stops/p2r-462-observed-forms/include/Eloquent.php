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

// Its parent is declared nowhere, so PHP throws when Laravel boots a model it observes.
final class Orphan extends MissingBase
{
}

trait WatchesTrait
{
}

enum Kind
{
}

// Its interface is declared nowhere.
final class NoInterface implements MissingInterface
{
}

// Its parent is built into PHP, which Mago's stubs cannot tell from a class of an extension
// that PHP may not load.
final class BuiltinChild extends \ArrayObject
{
}
