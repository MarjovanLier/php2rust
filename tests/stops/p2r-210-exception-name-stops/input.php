<?php

declare(strict_types=1);

namespace App;

final readonly class PhpException
{
    public function __construct(
        public int $n,
    ) {}
}

function local(int $n): int
{
    if ($n < 0) {
        throw new RuntimeException('negative');
    }
    return $n;
}

final readonly class Result {}

final readonly class Ok {}

enum Err
{
    case Failed;
}
