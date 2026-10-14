<?php

declare(strict_types=1);

namespace App\One;

final class Item
{
    public function __construct(public int $n) {}
}

function getTotal(): int
{
    return 1;
}

function report(): int
{
    return 2;
}
