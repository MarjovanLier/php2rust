<?php

declare(strict_types=1);

namespace Spatie\LaravelData;

// A Data of the input is a class of the input, so extending it stops as any extends.
class Data
{
    public const NAME = 'base';

    public function name(): string
    {
        return static::NAME;
    }
}

final class Point extends Data
{
    public const NAME = 'child';
}
