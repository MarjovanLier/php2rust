<?php

declare(strict_types=1);

function accent(string $word): int
{
    return match ($word) {
        'éééééééééééééééééééé', 'ëëëëëëëëëëëëëëëëëëëë', 'èèèèèèèèèèèèèèèèèèèè' => 1,
        default => 0,
    };
}

function wide(string $word): int
{
    return match ($word) {
        'a', 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb' => 1,
        default => 0,
    };
}

echo accent('é'), wide('a'), "\n";
