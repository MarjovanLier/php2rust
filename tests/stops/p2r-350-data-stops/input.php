<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

class Point extends Data
{
    public function __construct(
        public int $x,
    ) {}
}

// Data declares $_additional, so a property of that name stops.
class Extra extends Data
{
    public function __construct(
        public int $_additional,
    ) {}
}

// Only Data itself as the direct parent converts.
class Deeper extends Point
{
}

function parentCall(Point $p): int
{
    $p->toArray();
    return $p->x;
}

function viaFrom(): int
{
    return Point::from(1)->x;
}

function anyData(Data $d): int
{
    return 1;
}

function isData(Point $p): bool
{
    return $p instanceof Data;
}
