<?php

declare(strict_types=1);

namespace App;

use DateTime;
use Spatie\LaravelData\Optional;

// A class that changes its objects is never cloned.
final class Box
{
    public function __construct(public int $size)
    {
    }

    public function grow(): void
    {
        $this->size++;
    }
}

final class Holder
{
    public function __construct(public Box|Optional $box)
    {
    }
}

// A class that holds itself would have no size.
final readonly class Node
{
    public function __construct(public Node|Optional $next)
    {
    }
}

/** @param array{name: string|Optional} $row */
function shaped(array $row): int
{
    return 1;
}

/** @param array<string, int|Optional> $counts */
function mapped(array $counts): int
{
    return 1;
}

function dated(DateTime|Optional|null $when): int
{
    return 1;
}

function flagged(true|Optional $flag): int
{
    return 1;
}

function nothing(?Optional $marker): int
{
    return 1;
}
