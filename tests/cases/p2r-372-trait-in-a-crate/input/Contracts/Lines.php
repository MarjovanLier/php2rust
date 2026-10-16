<?php

declare(strict_types=1);

namespace App\Contracts;

interface Lines
{
    /** @return list<array{name: string, amount: int}> */
    public function lines(string $csv): array;

    public function amount(): int;
}
