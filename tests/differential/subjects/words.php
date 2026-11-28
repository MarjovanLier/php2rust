<?php

declare(strict_types=1);

namespace Words;

// Plain functions without Laravel, so the harness's own tests run in CI (P2R-609).
function shout(string $word): string
{
    echo 'shouting ', $word, "\n";
    return strtoupper($word) . '!';
}

function twice(int $n): int
{
    return $n * 2;
}

function bigger(float $x): bool
{
    return $x > 1.5;
}

/**
 * @return list<int>
 */
function upTo(int $n): array
{
    $found = [];
    foreach ([1, 2, 3, 4] as $i) {
        if ($i <= $n) {
            $found[] = $i;
        }
    }
    return $found;
}

function nameOrNull(string $name): ?string
{
    if ($name === '') {
        return null;
    }
    return $name;
}
