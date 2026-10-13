<?php

declare(strict_types=1);

readonly class Node
{
    public function __construct(public int $x)
    {
    }
}

readonly class Maybe
{
    public function __construct(public ?Node $node)
    {
    }
}

function swap(Node $n): int
{
    $first = $n->x;
    $n = new Node(5);
    return $first + $n->x;
}

function optional(?Node $n): int
{
    return 1;
}

function build(): int
{
    $m = new Maybe(null);
    return count([$m]);
}
