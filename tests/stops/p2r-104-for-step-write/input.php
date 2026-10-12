<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

for ($i = 0, $v = code(0); $i < 3; $i++, $v = code($i)) {
    if ($v === null) {
        continue;
    }
    echo $v, "\n";
}
