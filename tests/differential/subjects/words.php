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
