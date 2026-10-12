<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

$v = code(1);
for ($i = 0; $i < 30; $i = $i + $v) {
    if ($v === null) {
        continue;
    }
    echo $i, "\n";
}
