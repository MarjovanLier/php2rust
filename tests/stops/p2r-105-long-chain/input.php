<?php

declare(strict_types=1);

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

$t = name(0);
echo strlen($t ?? 'a default text that is far too long'), "\n";
