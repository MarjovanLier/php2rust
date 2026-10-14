<?php

declare(strict_types=1);

namespace Acme\Shapes;

use Acme\Shapes\Side;

enum Side
{
    case Left;
    case Right;
}

function flip(Side $side): Side
{
    if ($side === Side::Left) {
        return Side::Right;
    }
    return Side::Left;
}
