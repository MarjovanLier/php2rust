<?php

declare(strict_types=1);

use Lib\Money;

function total(Money $money): int
{
    return 1;
}

function counted(int $value): int
{
    return \Lib\cents(new Money($value));
}
