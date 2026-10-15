<?php

declare(strict_types=1);

namespace App\Model;

final readonly class Bag
{
    /**
     * @param array<string, int> $items
     */
    public function __construct(public array $items)
    {
    }

    public function weight(string $item): int
    {
        return $this->items[$item] ?? 0;
    }
}
