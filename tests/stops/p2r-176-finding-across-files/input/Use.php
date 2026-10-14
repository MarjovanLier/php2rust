<?php

declare(strict_types=1);

namespace App\Caller;

use function App\count_up;

function twice(string $s): int
{
    return count_up($s) * 2;
}
