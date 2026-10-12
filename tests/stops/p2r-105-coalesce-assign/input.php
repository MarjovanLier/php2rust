<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

$a = code(1);
$a ??= 5;
if ($a !== null) {
    echo "set\n";
}
