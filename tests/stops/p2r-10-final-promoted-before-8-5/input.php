<?php

declare(strict_types=1);

// PHP 8.3 and 8.4 fail to compile final on a promoted parameter; PHP 8.5 added it, and
// Mago parses it for every version.
final readonly class Point
{
    public function __construct(final int $x, public final int $y)
    {
    }
}

$point = new Point(1, 2);
echo $point->x + $point->y, "\n";
