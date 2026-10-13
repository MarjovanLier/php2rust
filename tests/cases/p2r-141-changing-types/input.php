<?php

declare(strict_types=1);

final class Counter
{
    public int $count = 0;
}

final class Panel
{
    public function __construct(public readonly Counter $counter)
    {
    }
}

readonly class Board
{
    /**
     * @param list<Counter> $counters
     */
    public function __construct(public array $counters)
    {
    }
}

final class Fixed
{
    public function __construct(public readonly int $size)
    {
    }
}

$panel = new Panel(new Counter());
$panel->counter->count = 4;
$board = new Board([new Counter(), new Counter()]);
$fixed = new Fixed(2);
$copy = $fixed;
echo $panel->counter->count, " ", count($board->counters), " ", $copy->size + $fixed->size, "\n";
