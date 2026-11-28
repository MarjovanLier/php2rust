<?php

declare(strict_types=1);

// PHP 8.4 added a set visibility such as private(set), and Mago's analysis set to PHP
// 8.3 reports it, so only that target stops.
final readonly class Counter
{
    public function __construct(public private(set) int $count)
    {
    }
}

echo new Counter(3)->count, "\n";
