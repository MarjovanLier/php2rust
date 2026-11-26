<?php

declare(strict_types=1);

/**
 * @param list<int> $counts
 */
function byName(array $counts, string $name): int
{
    return $counts[$name];
}

final class Lines
{
    /**
     * @param list<string> $items
     */
    public function __construct(public array $items)
    {
    }

    public function add(string $line): void
    {
        $this->items[] .= $line;
    }
}
