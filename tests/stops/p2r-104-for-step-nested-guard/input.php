<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

for ($i = 0, $c = code(0); $i < 5; $i++, $c = code($i)) {
    if ($i > 1) {
        if ($c === null) {
            continue;
        }
        if ($c > 2) {
            continue;
        }
        echo $c, "\n";
    }
}
