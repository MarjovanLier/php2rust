<?php

declare(strict_types=1);

/**
 * @param list<int> $list
 */
function casts(string $s, int $n, float $f, bool $b, array $list, ?string $maybe): void
{
    echo (int) $f;
    echo (int) $n;
    echo (int) $b;
    echo (string) $s;
    echo (string) $f;
    echo (integer) $s;
    echo (binary) $n;
    echo (bool) $n;
    echo (float) $s;
    echo count((array) $n);
    echo (int) $maybe;
    echo (string) $list;
    echo (int) null;
    echo (unset) $n;
    echo (object) $n;
    echo (string) Suit::Hearts;
}

enum Suit
{
    case Hearts;
}
