<?php

declare(strict_types=1);

/**
 * @param list<string> $list
 */
function check(?string $maybe, string $s, array $list): bool
{
    $a = str_starts_with($maybe, "a");
    $b = str_ends_with($s);
    $c = str_contains(haystack: $s, needle: "x");
    $d = str_contains($list, "x");
    return $a && $b && $c && $d;
}

if (check("1", "2", [])) {
    echo "y\n";
}
