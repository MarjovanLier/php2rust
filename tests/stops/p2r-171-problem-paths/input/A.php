<?php

declare(strict_types=1);

namespace App;

function outer(): int
{
    return intdiv(1, 0);
}
