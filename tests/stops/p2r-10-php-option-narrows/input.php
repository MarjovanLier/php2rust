<?php

declare(strict_types=1);

// php.txt names 8.3 only, so the stop that only 8.3 and 8.4 give prints without the
// versions in front.
final readonly class Point
{
    public function __construct(final int $x)
    {
    }
}

echo new Point(1)->x, "\n";
