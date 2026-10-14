<?php

declare(strict_types=1);

namespace App\Two;

enum Item
{
    case One;
}

function get_total(): int
{
    return 3;
}
