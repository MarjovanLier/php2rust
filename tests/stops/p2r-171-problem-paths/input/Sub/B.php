<?php

declare(strict_types=1);

namespace App\Sub;

function inner(): int
{
    return 1 % 0;
}

function missing(): int
{
    return unknown_function();
}
