<?php

declare(strict_types=1);

namespace App\A;

final class Tally
{
    public int $total = 0;
}

final class Counter
{
    public int $n = 0;

    public function inc(): void
    {
        $this->n++;
    }
}

final readonly class Debug
{
    public function __construct(public int $x) {}
}
