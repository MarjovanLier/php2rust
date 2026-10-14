<?php

declare(strict_types=1);

namespace Shop\Model;

enum Size
{
    case Small;
    case Box;
}

function total(int $n): int
{
    return $n + 1;
}
