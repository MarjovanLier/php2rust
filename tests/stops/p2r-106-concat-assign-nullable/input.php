<?php

declare(strict_types=1);

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

$s = name(1);
$s .= '!';
if ($s !== null) {
    echo "set\n";
}
