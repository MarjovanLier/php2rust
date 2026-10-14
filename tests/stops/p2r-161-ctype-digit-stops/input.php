<?php

declare(strict_types=1);

/**
 * @param list<string> $list
 */
function check(?string $maybe, string $s, array $list): bool
{
    $a = ctype_digit($maybe);
    $b = ctype_digit($s, $s);
    $c = ctype_digit(text: $s);
    $d = ctype_digit($list);
    return $a && $b && $c && $d;
}

if (check("1", "2", [])) {
    echo "y\n";
}
