<?php

declare(strict_types=1);

namespace Lib;

final class Money
{
    public function __construct(public readonly int $cents)
    {
    }
}

function cents(Money $money): int
{
    return $money->cents;
}
