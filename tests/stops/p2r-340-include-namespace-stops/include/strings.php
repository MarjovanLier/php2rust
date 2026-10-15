<?php

declare(strict_types=1);

namespace App;

// In namespace App an unqualified strlen() calls this function, not PHP's.
function strlen(string $text): string
{
    return $text;
}

const RATE = 15;
