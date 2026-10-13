<?php

declare(strict_types=1);

final class Cell
{
    public int $v = 0;
}

function nudge(Cell $cell): void
{
    $cell->v++;
}

function pick(): Cell
{
    return new Cell();
}

function fromCall(): void
{
    nudge(pick());
}
