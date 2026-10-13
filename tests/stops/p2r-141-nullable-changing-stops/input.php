<?php

declare(strict_types=1);

final class Cell
{
    public int $v = 0;
}

final class Grid
{
    /** @var list<Cell>|null */
    public ?array $cells = null;
}

/**
 * @param list<Cell>|null $cells
 */
function size(?array $cells): int
{
    if ($cells === null) {
        return 0;
    }
    return 1;
}

/**
 * @return list<Cell>|null
 */
function none(): ?array
{
    return null;
}

/**
 * @param list<Cell> $cells
 */
function local(array $cells): void
{
    $m = null;
    if (count($cells) > 0) {
        $m = $cells;
    }
    if ($m === null) {
        echo "none\n";
    }
}
