<?php

declare(strict_types=1);

namespace App\Processors;

use App\Contracts\Lines;

final readonly class Splitter implements Lines
{
    public function __construct(public int $amount)
    {
    }

    /** @return list<array{name: string, amount: int}> */
    public function lines(string $csv): array
    {
        $rows = [];
        foreach (explode(',', $csv) as $name) {
            $rows[] = ['name' => $name, 'amount' => $this->amount];
        }
        return $rows;
    }
}
